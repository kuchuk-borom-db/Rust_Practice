#[derive(Clone)]
pub struct AppState {
    pub services: AvailableServices,
}
#[derive(Clone)]
pub struct AvailableServices {
    pub persistence: crate::services::persistence::AvailableServices,
}
