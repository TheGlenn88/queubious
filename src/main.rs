use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use listenfd::ListenFd;
use log::info;
use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration as Dur;
use uuid::Uuid;

mod mobc_pool;

pub type MobcPool = Pool<RedisConnectionManager>;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    qid: String,
    exp: i64,
}

async fn index(_req: HttpRequest, mobc_pool: web::Data<MobcPool>) -> impl Responder {
    // check if there is a queue, if there is, create an ID and forward to /waiting-room
    // if there isn't a queue, generate a token and forward to the original referrer

    // TODO: move the value to an ENV variable. Or a redis value?
    let should_queue_limit: u32 = 100;

    let active_users = mobc_pool::llen(&mobc_pool, "active_users").await;

    // TODO: create a cookie and a JWT token for the referral
    let uuid = Uuid::new_v4();

    // check if the user should queue
    if active_users.unwrap() > should_queue_limit {
        let lpush = mobc_pool::lpush(
            &mobc_pool,
            "queue",
            (uuid.to_string()).as_str(),
        )
        .await;
        println!("{:?}", lpush);

        HttpResponse::Found()
            .header(http::header::LOCATION, "/waiting-room")
            .finish()
    } else {
        let iat = Local::now();
        let expp = iat + Duration::hours(i64::from(3600));

        let my_claims = Claims {
            sub: "blah".to_string(),
            qid: uuid.to_string(),
            exp: expp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret("12345".as_ref()),
        );

        println!("{:?}", token);
        let url = format!("http://localhost:3000?token={}", token.unwrap());

        HttpResponse::Found()
            .header(http::header::LOCATION, url)
            .finish()
    }
}

async fn status(_req: HttpRequest, mobc_pool: web::Data<MobcPool>) -> impl Responder {
    // check the redis cache directly if we are ready to check position

    // let x: u32 = 0;
    // let set_incr = mobc_pool::set_str(
    //     &mobc_pool,
    //     "incr",
    //     (x.to_string()).as_str(),
    //     60usize,
    // ).await;
    // println!("{:?}", set_incr);

    // test lpop
    // let lpop = mobc_pool::lpop(
    //     &mobc_pool,
    //     "queue"
    // ).await;
    // println!("{:?}", lpop);

    // for i in 0..100 {
    //     let uuid = Uuid::new_v4();
    //     let lpush = mobc_pool::lpush(
    //         &mobc_pool,
    //         "queue",
    //         (uuid.to_string()).as_str()
    //     ).await;
    //     println!("{:?}", lpush);
    // }

    // test incrementor

    // let incr = mobc_pool::incr(
    //     &mobc_pool,
    //     "incr",
    // ).await;
    // println!("{:?}", incr);

    // get value by key
    // let hit_cache = mobc_pool::get_str(
    //     &mobc_pool,
    //     "incr",
    // ).await;
    // println!("{:?}", hit_cache);
    HttpResponse::Ok().finish()
}

async fn waiting_room(_req: HttpRequest, mobc_pool: web::Data<MobcPool>) -> impl Responder {
    // serve the waiting room page, showing position in queue and the time until exit
    let hit_cache = mobc_pool::get_str(&mobc_pool, "key").await;
    println!("{:?}", hit_cache);
    HttpResponse::Ok()
        .content_type("text/html")
        .body("waiting, room")
}

async fn expend_token(
    _req: HttpRequest,
    producer: web::Data<FutureProducer>,
    mobc_pool: web::Data<MobcPool>,
) -> impl Responder {
    // TODO: Parse the token, extract the user, send a completed message to kafka for that user.
    // produce a kafka message
    let topic = "test";
    let delivery_status = producer
        .send(
            FutureRecord::to(topic)
                .payload(&format!("Message {}", "payload"))
                .key(&format!("Key {}", "key"))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            Dur::from_secs(0),
        )
        .await;
    info!("Future completed. Result: {:?}", delivery_status);

    // set redis key
    let result = mobc_pool::set_str(
        &mobc_pool,
        "key",
        "value",
        60usize,
    )
    .await
    .map_err(|e| {
        println!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    });
    println!("{:?}", result);

    // retrieve redis key
    let hit_cache = mobc_pool::get_str(&mobc_pool, "key").await;
    println!("{:?}", hit_cache);

    // return temporary 200
    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mobc_pool = mobc_pool::connect().await.expect("can create mobc pool");
    let mobc_pool = web::Data::new(mobc_pool);

    let brokers = "localhost:9092";
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    let producer = web::Data::new(producer);

    let mut server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/status", web::get().to(status))
            .route("/waiting-room", web::get().to(waiting_room))
            .route("/expend", web::get().to(expend_token))
            .app_data(producer.clone())
            .app_data(mobc_pool.clone())
    });

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8002")?
    };

    server.run().await
}
