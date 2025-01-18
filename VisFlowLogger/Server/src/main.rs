mod services;

fn main() {
    //Create the services
    let vis_flow_log_repo = services::persistence::api::services::vis_flow_log::new();
}
