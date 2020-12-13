use actix_web::{web, http, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_derive::{Serialize, Deserialize};
use chrono::{Duration, Local};
use std::time::Duration as Dur;
use log::info;


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

async fn index(_req: HttpRequest) -> impl Responder {
    let iat = Local::now();
    let expp = iat + Duration::hours(i64::from(3600));

    let my_claims = Claims {
        sub: "blah".to_string(),
        qid: "54331".to_string(),
        exp: expp.timestamp(),
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("12345".as_ref()));

    println!("{:?}", token);
    let url = format!("http://localhost:3000?token={}", token.unwrap());

    HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish()
}

async fn expend_token(_req: HttpRequest) -> impl Responder {
    // TODO: Parse the token, extract the user, send a completed message to kafka for that user.

    // produce a kafka message
    let topic = "test";
    let brokers = "localhost:9092";
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

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
        HttpResponse::Ok()
            .finish()
}

async fn check(_req: HttpRequest) -> impl Responder {
    // check the redis cache directly if we are ready to check position
    
    HttpResponse::Ok()
        .finish()
}

async fn produce(brokers: &str, topic_name: &str) {

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


        let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", 1))
                        .key(&format!("Key {}", 1))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                        Dur::from_secs(0),
                )
                .await;

        info!("Future completed. Result: {:?}", delivery_status);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(||
        App::new()
        .route("/", web::get().to(index))
        .route("/check", web::get().to(check))
        .route("/expend", web::get().to(expend_token))
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8002")?
    };

    server.run().await
}
