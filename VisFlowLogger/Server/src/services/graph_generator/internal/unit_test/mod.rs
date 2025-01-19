use crate::services::graph_generator::api::models::vis_flow_log_entry::{VisFlowLogEntry, VisFlowLogEntryLogType};
use crate::services::graph_generator::api::services::graph_generator::GraphGenerator;
#[test]
fn graph_generator_test() {
    let graph_gen = crate::services::graph_generator::api::services::graph_generator::new();
    let entries: Vec<VisFlowLogEntry> = vec![
        // Main starts
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Start,
            log_value: None,
            block_name: "main".parse().unwrap(),
        },
        // Main logs adding 2 and 1
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Log,
            log_value: Some("Adding 2 and 1".to_string()),
            block_name: "main".parse().unwrap(),
        },

        // Main starts sum for the first time
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Start,
            log_value: None,
            block_name: "sum".parse().unwrap(),
        },
        // Sum logs the calculation of 2 + 1 = 3
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Log,
            log_value: Some("2 + 1 = 3".to_string()),
            block_name: "sum".parse().unwrap(),
        },

        // Sum recursively calls sum
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Start,
            log_value: None,
            block_name: "sum".parse().unwrap(),
        },
        // Recursive sum logs the calculation of 1 + 1 = 2
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Log,
            log_value: Some("1 + 1 = 2".to_string()),
            block_name: "sum".parse().unwrap(),
        },

        // Recursive sum ends
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::End,
            log_value: None,
            block_name: "sum".parse().unwrap(),
        },

        // Sum ends after returning from the recursive call
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::End,
            log_value: None,
            block_name: "sum".parse().unwrap(),
        },

        // Main stores the result of sum
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Store,
            log_value: Some("sum = 3".to_string()),
            block_name: "main".parse().unwrap(),
        },

        // Main calls foo
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Start,
            log_value: None,
            block_name: "foo".parse().unwrap(),
        },
        // Foo logs its calculation
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::Log,
            log_value: Some("foo called".to_string()),
            block_name: "foo".parse().unwrap(),
        },
        // Foo ends
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::End,
            log_value: None,
            block_name: "foo".parse().unwrap(),
        },

        // Main ends
        VisFlowLogEntry {
            log_type: VisFlowLogEntryLogType::End,
            log_value: None,
            block_name: "main".parse().unwrap(),
        },
    ];
    /*
    [
      {
        "log_type": "Start",
        "log_value": null,
        "block_name": "main"
      },
      {
        "log_type": "Log",
        "log_value": "Adding 2 and 1",
        "block_name": "main"
      },
      {
        "log_type": "Start",
        "log_value": null,
        "block_name": "sum"
      },
      {
        "log_type": "Log",
        "log_value": "2 + 1 = 3",
        "block_name": "sum"
      },
      {
        "log_type": "Start",
        "log_value": null,
        "block_name": "sum"
      },
      {
        "log_type": "Log",
        "log_value": "1 + 1 = 2",
        "block_name": "sum"
      },
      {
        "log_type": "End",
        "log_value": null,
        "block_name": "sum"
      },
      {
        "log_type": "End",
        "log_value": null,
        "block_name": "sum"
      },
      {
        "log_type": "Store",
        "log_value": "sum = 3",
        "block_name": "main"
      },
      {
        "log_type": "Start",
        "log_value": null,
        "block_name": "foo"
      },
      {
        "log_type": "Log",
        "log_value": "foo called",
        "block_name": "foo"
      },
      {
        "log_type": "End",
        "log_value": null,
        "block_name": "foo"
      },
      {
        "log_type": "End",
        "log_value": null,
        "block_name": "main"
      }
    ]
    */

    let generated = graph_gen.generate_graph(entries).unwrap();
    generated.iter().for_each(|(k, v)| {
        println!("{} : {}", k, v);
    })
}
