use crate::services::diagram_generator::api::services::DiagramType;
use crate::services::diagram_generator::AvailableServices;
use crate::services::graph_generator::AvailableServices as GGAS;
use crate::services::persistence::api::services::vis_flow_op::VisFlowOp;
use crate::services::persistence::AvailableServices as PAS;
use actix_web::{web, App, HttpServer};
use server::models::app_state::{AppState, AvailableServices as AS};
use std::sync::Arc;

mod server;
mod services;
mod utils;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Create the services
    let vis_flow_log_repo = services::persistence::api::services::vis_flow_log::new().await;
    let vis_flow_op_repo = services::persistence::api::services::vis_flow_op::new().await;
    let graph_generator = services::graph_generator::api::services::graph_generator::new();
    let diagram_generator = services::diagram_generator::api::services::new(DiagramType::Mermaid);

    let app_state = AppState {
        services: AS {
            persistence: PAS {
                vis_flow_log: Arc::new(vis_flow_log_repo),
                vis_flow_op: Arc::new(vis_flow_op_repo),
            },
            graph_generator: GGAS {
                graph_generator: Arc::new(graph_generator),
            },
            diagram_generator: AvailableServices {
                mermaid: Arc::new(diagram_generator),
            },
        },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(server::route::save_logs)
            .service(server::route::get_logs_by_operation_id)
            .service(server::route::get_operations)
            .service(server::route::get_graphs_by_operation_id)
            .service(server::route::generate_diagram_for_operation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
Box<T>
- Used for single ownership of a value stored on the heap.
- Allows mutation if the Box itself is mutable.

Rc<T> (Reference Counted)
- Allows multiple ownership of the same data within a single thread.
- Provides shared (immutable) references to the inner value.
- If mutation is needed, use Rc<RefCell<T>>.

Arc<T> (Atomic Reference Counted)
- Similar to Rc<T>, but designed for thread-safe multiple ownership.
- Provides shared (immutable) references to the inner value.
- For mutation in a thread-safe context, use Arc<Mutex<T>> or Arc<RwLock<T>>.

Arc<Mutex<T>>
- Combines Arc for thread-safe multiple ownership and Mutex for interior mutability.
- Allows safe, mutable access to the inner value across threads via mutual exclusion.

Cell<T>
- Provides interior mutability for types that implement the Copy trait.
- Enables modifying the value inside a Cell even with an immutable reference (&T).

Notes:
- RefCell<T>: Often paired with Rc<T> for interior mutability in single-threaded contexts.
- Interior Mutability: A design pattern that allows data to be modified through an immutable reference, using wrappers like Cell or RefCell.
*/
