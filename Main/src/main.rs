mod services;
mod utils;
fn generate_test_log() -> Vec<VisLog> {
    let logs = vec![
        VisLog {
            id: String::from("a"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("b"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::LOG,
            value: Some(String::from("num = 2")),
        },
        VisLog {
            id: String::from("c"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("d"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("e"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("f"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("g"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("h"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("i"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::LOG,
            value: Some(String::from("return 0")),
        },
        VisLog {
            id: String::from("j"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("k"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::LOG,
            value: Some(String::from("0")),
        },
        VisLog {
            id: String::from("l"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("m"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("n"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Some(String::from("return 0")),
        },
        VisLog {
            id: String::from("o"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("p"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::LOG,
            value: Some(String::from("0+0 = 0")),
        },
        VisLog {
            id: String::from("q"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("r"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("s"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("t"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::LOG,
            value: Some(String::from("return 4")),
        },
        VisLog {
            id: String::from("u"),
            operation_id: "".to_string(),
            name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("v"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::LOG,
            value: Some(String::from("4")),
        },
        VisLog {
            id: String::from("w"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        VisLog {
            id: String::from("x"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("y"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Some(String::from("return 20")),
        },
        VisLog {
            id: String::from("z"),
            operation_id: "".to_string(),
            name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("1"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::LOG,
            value: Some(String::from("24+4 = 24")),
        },
        VisLog {
            id: String::from("2"),
            operation_id: "".to_string(),
            name: String::from("processData"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("3"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::LOG,
            value: Some(String::from("process_data(2) = 24")),
        },
        VisLog {
            id: String::from("4"),
            operation_id: "".to_string(),
            name: String::from("add2"),
            log_type: LogType::START,
            value: None,
        },
        VisLog {
            id: String::from("5"),
            operation_id: "".to_string(),
            name: String::from("add2"),
            log_type: LogType::LOG,
            value: Some(String::from("return 3")),
        },
        VisLog {
            id: String::from("6"),
            operation_id: "".to_string(),
            name: String::from("add2"),
            log_type: LogType::END,
            value: None,
        },
        VisLog {
            id: String::from("7"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::ExternalCallStore,
            value: Some(String::from("Db repo")),
        },
        VisLog {
            id: String::from("8"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::ExternalCall,
            value: Some(String::from("External API Call")),
        },
        VisLog {
            id: String::from("9"),
            operation_id: "".to_string(),
            name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];
    logs
}

use crate::services::graph_generator::repo::api::model::{LogType, VisLog};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize};

#[derive(Deserialize)]
struct LogEntry {
    operation_id: String,
    log_name: String,
    log_type: String,
    log_value: String,
}

async fn index() -> impl Responder {
    "Hello world!"
}

async fn create_log_entry(log: web::Json<LogEntry>) -> impl Responder {
    // Here you can do something with the data, like logging it or processing it
    HttpResponse::Ok().json(log.into_inner())  // Echo back the received JSON
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                // Prefixes all resources and routes attached to it...
                web::scope("")
                    // Handle requests for `GET /gg`
                    .route("/gg", web::get().to(index))
                    // Handle POST request for `/log_entry`
                    .route("/log_entry", web::post().to(create_log_entry)),
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
