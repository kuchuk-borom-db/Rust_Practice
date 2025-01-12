mod services;
mod utils;

use crate::services::diagram_generator;
use crate::services::diagram_generator::api::service::DiagramGeneratorTrait;
use crate::services::graph_generator::api::GraphGeneratorTrait;
use crate::services::graph_generator::repo::api::model::{LogType, VisLog};
use crate::services::graph_generator::*;
use crate::services::graph_generator::*;
use simple_logger::SimpleLogger;

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
fn main() -> () {
    SimpleLogger::new().init().unwrap();

    //List of fake entries

    let graph_gen = api::Factory::factory();
    let result = graph_gen.generate_graph(generate_test_log());
    let diagram_gen = diagram_generator::api::service::Factory::factory();
    let generated_diagram = diagram_gen.generate_diagram(result.as_ref().unwrap());

    println!("{:?}", result.unwrap());
    println!("{}", generated_diagram.unwrap());
}
