pub mod model {
    #[derive(PartialEq, Debug, Clone)]
    pub enum LogType {
        LOG,
        STORE,
        START,
        END,
        ExternalCall,
        ExternalCallStore,
    }
    #[derive(Debug)]
    pub struct VisLog {
        pub id: String,
        pub operation_id: String,
        pub name: String,
        pub log_type: LogType,
        pub value: Option<String>,
    }
}
