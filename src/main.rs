use actix_cors::Cors;
use actix_rt::Arbiter;
use actix_session::CookieSession;
use actix_session::Session;
use actix_web::{
    http, http::header, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use base64::decode as b64decode;
use chrono::{Duration, Local};
use deadpool_redis::{cmd, Config as RedisConfig, Pool};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use listenfd::ListenFd;
use redis::RedisError;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::thread;
use std::time::Duration as Dur;
use tera::Context;
use tera::Tera;
use uuid::Uuid;

use actix_files::NamedFile;
use std::path::PathBuf;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    qid: String,
    aud: String,
    iat: i64,
    nbf: i64,
    exp: i64,
    cexp: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    position: usize,
    progress: usize,
    wait_time: String,
    last_updated: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    timestamp: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Heartbeat {
    token: String,
}

pub struct AppData {
    pub tmpl: Tera,
}

fn redis_uri() -> String {
    match env::var("REDIS_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from(String::from("redis://127.0.0.1:6379")),
    }
}

// async fn redis_ping(pool: &Pool) -> Result<String, PoolError> {
//     let mut connection: Connection = pool.get().await?;
//     let pong: String = cmd("PING").query_async(&mut connection).await?;

//     Ok(pong)
// }

async fn terminate(redis_pool: web::Data<Pool>, id: &str) -> usize {
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("LREM")
        .arg(&["active", "0", &id.to_string()])
        .query_async(&mut redis_conn)
        .await
        .unwrap()
}

async fn push_to_queue(redis_pool: web::Data<Pool>, id: Uuid) -> usize {
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("RPUSH")
        .arg(&["queue", &id.to_string()])
        .query_async(&mut redis_conn)
        .await
        .unwrap()
}

async fn push_to_active(redis_pool: web::Data<Pool>, id: Uuid) -> usize {
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("RPUSH")
        .arg(&["active", &id.to_string()])
        .query_async(&mut redis_conn)
        .await
        .unwrap()
}

async fn get_queue_length(redis_pool: web::Data<Pool>) -> usize {
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("LLEN")
        .arg(&["queue"])
        .query_async(&mut redis_conn)
        .await
        .unwrap()
}

async fn get_queue_position(redis_pool: web::Data<Pool>, id: Uuid) -> Result<usize, RedisError> {
    let mut redis_conn = redis_pool.get().await.unwrap();

    let result: Result<usize, RedisError> = cmd("LPOS")
        .arg(&["queue", &id.to_string()])
        .query_async(&mut redis_conn)
        .await;

    println!("LPOS {:?}", result);

    result
}

async fn get_active_length(redis_pool: web::Data<Pool>) -> usize {
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("LLEN")
        .arg(&["active"])
        .query_async(&mut redis_conn)
        .await
        .unwrap()
}

async fn get_status(session: Session, redis_pool: web::Data<Pool>) -> Status {
    let main_id;
    if let Some(id) = session.get::<String>("id").unwrap() {
        main_id = Uuid::parse_str(id.as_str()).unwrap();
        let queue_position = get_queue_position(redis_pool.clone(), main_id)
            .await
            .unwrap();

        let original_position;
        if let Some(pos) = session.get::<usize>("original_position").unwrap() {
            original_position = pos;
        } else {
            original_position = queue_position;
        }
        let percentage: f32 = 100.0 - ((queue_position as f32 / original_position as f32) * 100.0);

        println!("original pos: {}", original_position);
        println!("queue pos: {}", queue_position);
        println!("percentage {}", percentage);

        Status {
            position: queue_position + 1,
            progress: percentage as usize,
            // todo: will need to crunch queue egress data to get an average wait time.
            wait_time: String::from("119 Minutes"),
            last_updated: Local::now().time().format("%H:%M:%S").to_string(),
            messages: Vec::new(),
        }
    } else {
        Status {
            position: 1,
            progress: 0 as usize,
            // todo: will need to crunch queue egress data to get an average wait time.
            wait_time: String::from("119 Minutes"),
            last_updated: Local::now().time().format("%H:%M:%S").to_string(),
            messages: Vec::new(),
        }
    }
}

async fn heartbeat(
    _req: HttpRequest,
    data: web::Json<Heartbeat>,
    redis_pool: web::Data<Pool>,
    _producer: web::Data<FutureProducer>,
) -> impl Responder {
    let token: TokenData<Claims>;
    match decode(
        &data.token,
        &DecodingKey::from_base64_secret(&env::var("JWT_SECRET").unwrap()).unwrap(),
        &Validation::default(),
    ) {
        Ok(tok) => token = tok,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut redis_conn = redis_pool.get().await.unwrap();

    let exists: bool = cmd("EXISTS")
        .arg(&[&token.claims.qid])
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    if exists {
        let timeout: String = (env::var("ACTIVE_SESSION_TIMEOUT")
            .unwrap()
            .parse::<u16>()
            .unwrap()
            * 60 as u16)
            .to_string();
        cmd("EXPIRE")
            .arg(&[&token.claims.qid, &timeout])
            .execute_async(&mut redis_conn)
            .await
            .unwrap();
    }
    //TODO: store the initial time the key was set in here, so that you can give a MAX session time if needed
    //TODO: move the initial setting of this key to when moving out of the queue, or bypassing the queue
    // cmd("SET")
    //     .arg(&[&token.claims.qid, &"1".to_string()])
    //     .execute_async(&mut redis_conn)
    //     .await
    //     .unwrap();

    HttpResponse::Ok().finish()
}

async fn script(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("app_url", &env::var("APP_URL").unwrap());
    let rendered = data.tmpl.render("queubious.js", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn index(
    _req: HttpRequest,
    session: Session,
    redis_pool: web::Data<Pool>,
    producer: web::Data<FutureProducer>,
) -> impl Responder {
    let mut redis_conn = redis_pool.get().await.unwrap();

    // get the active user limit from the redis cache, if the amount of active users exceeds this number, users will have to queue
    let should_queue_limit: usize = cmd("GET")
        .arg(&["active_user_limit"])
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    let queue_length: usize = get_queue_length(redis_pool.clone()).await;
    let active_users: usize = get_active_length(redis_pool.clone()).await;

    let main_id: Uuid;
    if let Some(id) = session.get::<String>("id").unwrap() {
        main_id = Uuid::parse_str(id.as_str()).unwrap();
    } else {
        main_id = Uuid::new_v4();
        session.set("id", main_id.to_string()).unwrap();
    }

    // check if the user should queue
    // TODO: check if the user is in active users
    if queue_length >= 1 || active_users >= should_queue_limit {
        println!("queueing {}", queue_length);
        if let None = session.get::<bool>("should_queue").unwrap() {
            // produce a test kafka message to add to queue
            let topic = "queue";
            let _kafka_result = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&main_id.to_string())
                        .key("add"),
                    Dur::from_secs(0), // TODO: check this
                )
                .await
                .unwrap();

            let position = push_to_queue(redis_pool, main_id).await;
            session.set("original_position", position).unwrap();
            session.set("should_queue", true).unwrap();
        }

        HttpResponse::Found()
            .header(http::header::LOCATION, "/waiting-room")
            .finish()
    } else {
        println!("bypassing queue {}", queue_length);

        push_to_active(redis_pool.clone(), main_id).await;

        cmd("SET")
            .arg(&[&main_id.to_string(), &Local::now().to_string()])
            .execute_async(&mut redis_conn)
            .await
            .unwrap();

        let timeout: String = (env::var("ACTIVE_SESSION_TIMEOUT")
            .unwrap()
            .parse::<u16>()
            .unwrap()
            * 60 as u16)
            .to_string();

        cmd("EXPIRE")
            .arg(&[&main_id.to_string(), &timeout])
            .execute_async(&mut redis_conn)
            .await
            .unwrap();

        let iat = Local::now();
        let exp = iat + Duration::minutes(i64::from(20));

        let my_claims = Claims {
            sub: "queue-egress".to_string(),
            iss: env::var("APP_URL").unwrap(),
            aud: "http://localhost:8000".to_string(),
            qid: main_id.to_string(),
            iat: iat.timestamp(),
            nbf: iat.timestamp(),
            exp: exp.timestamp(),
            cexp: env::var("ACTIVE_SESSION_TIMEOUT")
                .unwrap()
                .parse::<u16>()
                .unwrap(),
        };

        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_base64_secret(&env::var("JWT_SECRET").unwrap()).unwrap(),
        );

        // TODO: forward to original referrer, store that in the cookie?
        let url = format!("http://localhost:8000?queubioustoken={}", token.unwrap());

        HttpResponse::Found()
            .header(http::header::LOCATION, url)
            .finish()
    }
}

async fn status(
    _req: HttpRequest,
    redis_pool: web::Data<Pool>,
    session: Session,
) -> impl Responder {
    let mut _redis_conn = redis_pool.get().await.unwrap();
    // TODO: check the active_users cache to see if uuid value exists in it. if it does, forward to referrer
    // TODO: if not, check the position of the user, and respond with it.

    if let Some(_) = session.get::<String>("id").unwrap() {
        HttpResponse::Ok().json(get_status(session, redis_pool).await)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

async fn waiting_room(
    _req: HttpRequest,
    session: Session,
    redis_pool: web::Data<Pool>,
    data: web::Data<AppData>,
) -> impl Responder {
    //TODO: remove the waiting_room function and combine this into the index function
    let status = get_status(session, redis_pool).await;
    let mut ctx = Context::new();
    ctx.insert("position", &status.position);
    ctx.insert("progress", &status.progress);
    ctx.insert("wait_time", &status.wait_time);
    ctx.insert("last_updated", &status.last_updated);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn terminate_session(
    req: HttpRequest,
    producer: web::Data<FutureProducer>,
    redis_pool: web::Data<Pool>,
) -> impl Responder {
    // TODO: if there is a session, parse that, void the user
    // TODO: if there is a token, parse that, void the user

    let session_id = req.match_info().get("session_id").unwrap();
    let _qty_terminated = terminate(redis_pool, session_id).await;

    // produce a test kafka message
    let topic = "terminate_session";
    let _delivery_status = producer
        .send(
            FutureRecord::to(topic)
                .payload(&format!("Message {}", "payload"))
                .key(&format!("Key {}", "key")),
            Dur::from_secs(0),
        )
        .await;
    // return temporary 200
    HttpResponse::Ok().finish()
}

async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let redis_config = RedisConfig {
        url: Some(redis_uri()),
        pool: None,
    };
    let redis_pool = web::Data::new(redis_config.create_pool().unwrap());
    let mut redis_conn = redis_pool.get().await.unwrap();

    cmd("SET")
        .arg(&[
            "active_user_limit",
            &env::var("DEFAULT_ACTIVE_USER_LIMIT").unwrap(),
        ])
        .execute_async(&mut redis_conn)
        .await
        .unwrap();

    // setup kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &env::var("KAFKA_BROKERS").unwrap())
        .set(
            "message.timeout.ms",
            &env::var("KAFKA_MESSAGE_TIMEOUT").unwrap(),
        )
        .create()
        .expect("Failed to create Kafka Producer");
    let producer = web::Data::new(producer);

    let tera = Tera::new(format!("{}", env::var("TEMPLATE_DIR").unwrap()).as_str()).unwrap();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::signed(&b64decode(env::var("APP_KEY").unwrap()).unwrap())
                    //TODO investivate locking to APP_URL
                    // .domain("http://localhost")
                    // .domain(env::var("APP_URL").unwrap())
                    .name("queubious_session")
                    .path("/")
                    .secure(false),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/", web::get().to(index))
            .route("/status", web::get().to(status))
            .route("/queubious.js", web::get().to(script))
            .route("/heartbeat", web::post().to(heartbeat))
            .route("/waiting-room", web::get().to(waiting_room))
            .route("/terminate/{session_id}", web::get().to(terminate_session))
            .route("/{filename:.*}", web::get().to(static_files))
            .data(AppData { tmpl: tera.clone() })
            .app_data(producer.clone())
            .app_data(redis_pool.clone())
    });

    let bind_ip = match env::var("BIND_IP") {
        Ok(ip) => ip,
        _ => String::from("0.0.0.0"),
    };

    let bind_port = match env::var("BIND_PORT") {
        Ok(port) => port,
        _ => String::from("80"),
    };

    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(format!("{}:{}", bind_ip, bind_port))?
    };

    Arbiter::spawn(async move {
        loop {
            /**
             * TODO: retrieve entire active list, loop the list and check dirctly for it's key/string value
             * if it exists, leave it, if it doesn't exist, remove the key from active and push in a new key
             * from the queue (if the queue exists)
             */
            let active: Vec<String> = cmd("LRANGE")
                .arg(&["active", "0", "100"])
                .query_async(&mut redis_conn)
                .await
                .unwrap();

            for a in active {
                let exists: bool = cmd("EXISTS")
                    .arg(&[&a])
                    .query_async(&mut redis_conn)
                    .await
                    .unwrap();
                if !exists {
                    println!("removing from active {}", &a);
                    cmd("LREM")
                        .arg(&["active", "0", &a])
                        .query_async(&mut redis_conn)
                        .await
                        .unwrap()
                }
            }

            let active_user_limit: usize = cmd("GET")
                .arg(&["active_user_limit"])
                .query_async(&mut redis_conn)
                .await
                .unwrap();

            let active_length: usize = cmd("LLEN")
                .arg(&["active"])
                .query_async(&mut redis_conn)
                .await
                .unwrap();

            let timeout: String = (env::var("ACTIVE_SESSION_TIMEOUT")
                .unwrap()
                .parse::<u16>()
                .unwrap()
                * 60 as u16)
                .to_string();

            if active_length < active_user_limit {
                println!("shorter");
                let qty_to_move = active_user_limit - active_length;
                for _ in 0..qty_to_move {
                    // TODO: LMOVE is only in redis 6.2...
                    let user: String = cmd("LMOVE")
                        .arg(&["queue", "active", "LEFT", "RIGHT"])
                        .query_async(&mut redis_conn)
                        .await
                        .unwrap();
                    println!("moving, {}", user);
                    cmd("SET")
                        .arg(&[&user, &Local::now().to_string()])
                        .execute_async(&mut redis_conn)
                        .await
                        .unwrap();
                    cmd("EXPIRE")
                        .arg(&[&user, &timeout])
                        .execute_async(&mut redis_conn)
                        .await
                        .unwrap();
                }
            }
            thread::sleep(Dur::from_millis(1000));
        }
    });

    println!("started server on port {}", bind_port);

    server.run().await
}
