use actix_web::web;
use crate::core::shared::models::vis_flow_log::VisFlowLogEntity;
use crate::core::web::api::models::AppState;

pub struct VisFlowLogRepo;
pub struct VisFlowOperationRepo;
pub trait TRepo<T> {
    async fn save(entity: T);
}
impl TRepo<VisFlowLogEntity> for VisFlowLogRepo {
    async fn save(entity: VisFlowLogEntity) {
        let app_state = web::Data::<AppState>::get_ref();
        todo!()
    }
}
