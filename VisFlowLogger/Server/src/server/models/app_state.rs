#[derive(Clone)]
pub struct AppState {
    pub services: AvailableServices,
}
#[derive(Clone)]
pub struct AvailableServices {
    pub persistence: crate::services::persistence::AvailableServices,
    pub graph_generator: crate::services::graph_generator::AvailableServices,
}
