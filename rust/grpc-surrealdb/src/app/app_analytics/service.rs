use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use my_macros::FieldsGetter;
use tonic::{Request, Response, Status};
pub mod app_analytics {
    tonic::include_proto!("app_analytics");
}

use app_analytics::{
    app_analytics_server::{AppAnalytics, AppAnalyticsServer},
    CreateUserAppEventRequest, GetAllUserAppEventsRequest, GetAllUserAppEventsResponse,
    GetUserAppEventRequest, UserAppEventResponse,
};
use validator::Validate;

use super::UserAppEvent;

#[derive(Debug, Default)]
pub struct AnalyticsService;

type TonicResult<T> = anyhow::Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl AppAnalytics for AnalyticsService {
    async fn create_user_app_event(
        &self,
        request: Request<CreateUserAppEventRequest>,
    ) -> TonicResult<UserAppEventResponse> {
        // let CreateUserAppEventRequest {
        //     user_id,
        //     page,
        //     event_name,
        //     description,
        // } = request.into_inner();

        // let user_id = user_id.parse::<uuid::Uuid>().map_err(|e| {
        //     log::error!("{e}");
        //     Status::internal("Problem parsing uuid to string")
        // })?;

        // let mut user_app_event = UserAppEvent::builder()
        //     .user_id(user_id)
        //     .page(page)
        //     .event_name(event_name)
        //     .description(description)
        //     .build();

        // user_app_event
        //     .validate()
        //     .map_err(|e| Status::invalid_argument(format!("Invalid argument {:?}", e)))?;

        // user_app_event
        //     .save(&db)
        //     .await
        //     .map_err(|_| Status::not_found("User not found"))?;

        // let id = user_app_event
        //     .id
        //     .map(|id| id.to_string())
        //     .ok_or_else(|| Status::internal("Problem parsing uuid to string"))?;

        // Ok(Response::new(UserAppEventResponse {
        //     id,
        //     user_id: user_app_event.user_id.to_string(),
        //     page: user_app_event.page,
        //     event_name: user_app_event.event_name,
        //     description: user_app_event.description,
        // }))
        todo!()
    }

    async fn get_user_app_event(
        &self,
        request: Request<GetUserAppEventRequest>,
    ) -> TonicResult<UserAppEventResponse> {
        let GetUserAppEventRequest { event_id, user_id } = request.into_inner();

        // let db = establish_connection().await;

        // let user_found = UserAppEvent::find_one(uuid)
        // .await
        // .map_err(|_| Status::not_found("User event not found"))?;

        // match user_found {
        //     Some(user) => {
        //         let id = user.id.expect("Problem getting user event id").to_string();
        //         let user_app_event_response = UserAppEventResponse {
        //             id,
        //             user_id: user.user_id.to_string(),
        //             event_name: user.event_name,
        //             page: user.page,
        //             description: user.description,
        //         };
        //         return Ok(Response::new(user_app_event_response));
        //     }
        //     None => return Err(Status::not_found("message")),
        // }
        todo!()
    }

    async fn get_all_user_app_events(
        &self,
        request: Request<GetAllUserAppEventsRequest>,
    ) -> TonicResult<GetAllUserAppEventsResponse> {
        let GetAllUserAppEventsRequest { user_id } = request.into_inner();

        // let db = establish_connection().await;

        // let user_app_event_keys = UserAppEvent::get_fields_serialized();
        // let user_app_event_found =
        //     UserAppEvent::find(&db user_id)
        //         .await
        //         .map_err(|_| Status::not_found("User data not found"))
        //         .expect("Problem finding user");

        // let user_app_event = user_app_event_found
        //     .try_collect::<Vec<UserAppEvent>>()
        //     .await
        //     .map_err(|_| Status::aborted("problem converting"))?
        //     .into_iter()
        //     .map(|event| {
        //         let id = event.id.expect("Problem getting event id").to_string();
        //         UserAppEventResponse {
        //             id,
        //             user_id: event.user_id.to_string(),
        //             event_name: event.event_name,
        //             page: event.page,
        //             description: event.description,
        //         }
        //     })
        //     .collect::<Vec<UserAppEventResponse>>();

        // Ok(Response::new(GetAllUserAppEventsResponse {
        //     user_app_event,
        // }))
        todo!()
    }
}

pub struct AnalyticsApp;

impl AnalyticsApp {
    pub fn get_server() -> AppAnalyticsServer<AnalyticsService> {
        AppAnalyticsServer::new(AnalyticsService::default())
    }
}
