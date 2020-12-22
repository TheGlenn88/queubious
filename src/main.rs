use actix_session::CookieSession;
use actix_session::Session;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use listenfd::ListenFd;
use log::info;
use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration as Dur;
use tera::Context;
use tera::Tera;
use uuid::Uuid;

use actix_files::NamedFile;
use std::path::PathBuf;

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
#[derive(Debug, Serialize, Deserialize)]
struct Status {
    position: usize,
}
pub struct AppData {
    pub tmpl: Tera,
}

async fn index(
    _req: HttpRequest,
    session: Session,
    mobc_pool: web::Data<MobcPool>,
) -> impl Responder {
    // check if there is a queue, if there is, create an ID and forward to /waiting-room
    // if there isn't a queue, generate a token and forward to the original referrer

    // TODO: move the value to an ENV variable. Or a redis value?
    let should_queue_limit: u32 = 100;

    let active_users = mobc_pool::llen(&mobc_pool, "active_users").await;

    let main_id: Uuid;

    if let Some(id) = session.get::<String>("id").unwrap() {
        main_id = Uuid::parse_str(id.as_str()).unwrap();
    } else {
        main_id = Uuid::new_v4();
        session.set("id", main_id.to_string()).unwrap(); //TODO: don't just unwrap
    }
    println!("{:?}", main_id);

    // check if the user should queue
    if active_users.unwrap() > should_queue_limit {
        if let Some(should_queue) = session.get::<bool>("should_queue").unwrap() {
            println!("{:?}", "found key in cookie");
        } else {
            let lpush = mobc_pool::lpush(&mobc_pool, "queue", (main_id.to_string()).as_str()).await;
            session.set("should_queue", true); //TODO: don't just unwrap
            println!("{:?}", "did not find key in cookie, pushing to redis");
            println!("{:?}", lpush);
        }

        HttpResponse::Found()
            .header(http::header::LOCATION, "/waiting-room")
            .finish()
    } else {
        let iat = Local::now();
        let expp = iat + Duration::hours(i64::from(3600));

        let my_claims = Claims {
            sub: "sub".to_string(),
            qid: main_id.to_string(),
            exp: expp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret("12345".as_ref()),
        );

        // TODO: forward to original referrer, store that in the cookie?
        let url = format!("http://localhost:3000?token={}", token.unwrap());

        HttpResponse::Found()
            .header(http::header::LOCATION, url)
            .finish()
    }
}

async fn status(_req: HttpRequest, mobc_pool: web::Data<MobcPool>) -> impl Responder {
    // TODO: check the active_users cache to see if uuid value exists in it. if it does, forward to referrer

    // TODO: if not, check the position of the user, and respond with it.

    // test sending a fixed response of position 999
    HttpResponse::Ok().json(Status { position: 999usize })
}

async fn waiting_room(
    _req: HttpRequest,
    _mobc_pool: web::Data<MobcPool>,
    data: web::Data<AppData>,
) -> impl Responder {
    // TODO: read ID from cookie
    // TODO: use LPOS to get the queue position
    let mut ctx = Context::new();
    ctx.insert("position", &1234);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn expend_token(
    _req: HttpRequest,
    producer: web::Data<FutureProducer>,
    mobc_pool: web::Data<MobcPool>,
) -> impl Responder {
    // TODO: Parse the token, extract the user, send a completed message to kafka for that user.

    // produce a test kafka message
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

    // set a redis key
    let result = mobc_pool::set_str(&mobc_pool, "key", "value", 60usize)
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

async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mobc_pool = mobc_pool::connect().await.expect("can create mobc pool");
    let mobc_pool = web::Data::new(mobc_pool);

    // setup kafka producer
    let brokers = "localhost:9092"; // TODO: env this value
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka Producer");
    let producer = web::Data::new(producer);

    // let tera = Tera::new(format!("{}{}", dotenv!("TEMPLATE_DIR"), "/templates/**/*").as_str())
    let tera = Tera::new(format!("{}", "templates/**/*").as_str()).unwrap();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain("http://localhost")
                    .name("queue_s")
                    .path("/")
                    .secure(false),
            )
            .route("/", web::get().to(index))
            .route("/status", web::get().to(status))
            .route("/waiting-room", web::get().to(waiting_room))
            .route("/expend", web::get().to(expend_token))
            .route("/build/{filename:.*}", web::get().to(static_files))
            .data(AppData { tmpl: tera.clone() })
            .app_data(producer.clone())
            .app_data(mobc_pool.clone())
    });

    // TODO: setup host and port as env
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8002")?
    };

    server.run().await
}
