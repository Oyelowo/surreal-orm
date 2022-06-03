use anyhow::{Context, Result};
use bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneOptions, ReadConcern};
use tonic::{Response, Status};
pub mod app_analytics {
    tonic::include_proto!("app_analytics");
}

use app_analytics::{
    app_analytics_server::{AppAnalytics, AppAnalyticsServer},
    CreateUserAppEventRequest, GetAllUserAppEventsRequest, GetAllUserAppEventsResponse,
    GetUserAppEventRequest, UserAppEventResponse,
};
use validator::Validate;
use wither::Model;

use crate::configs::{establish_connection, model_cursor_to_vec};

use super::UserAppEvent;

#[derive(Debug, Default)]
pub struct AnalyticsService;

#[tonic::async_trait]
impl AppAnalytics for AnalyticsService {
    async fn create_user_app_event(
        &self,
        request: tonic::Request<app_analytics::CreateUserAppEventRequest>,
    ) -> anyhow::Result<tonic::Response<app_analytics::UserAppEventResponse>, tonic::Status> {
        // let p =tonic::Status::not_found("r");
        let CreateUserAppEventRequest {
            user_id,
            page,
            event_name,
            description,
        } = request.into_inner();
        let db = establish_connection().await;

        let user_id = user_id
            .parse::<uuid::Uuid>()
            .with_context(|| "Could not parse Id").expect("shouldn't happen but change");

        let mut user_app_event = UserAppEvent::builder()
            .user_id(user_id)
            .page(page)
            .event_name(event_name)
            .description(description)
            .build();

        user_app_event
            .validate()
            .map_err(|e| Status::invalid_argument(format!("Invalid argument {:?}", e)))?;

        user_app_event
            .save(&db, None)
            .await
            .map_err(|_| Status::not_found("User not found"))?;
        let id = user_app_event.id.expect("id not found").to_string();

        Ok(Response::new(UserAppEventResponse {
            id,
            user_id: user_app_event.user_id.to_string(),
            page: user_app_event.page,
            event_name: user_app_event.event_name,
            description: user_app_event.description,
        }))
    }

    async fn get_user_app_event(
        &self,
        request: tonic::Request<app_analytics::GetUserAppEventRequest>,
    ) -> Result<tonic::Response<app_analytics::UserAppEventResponse>, tonic::Status> {
        let GetUserAppEventRequest { event_id, user_id } = request.into_inner();

        let db = establish_connection().await;

        // Validate that it is being called by authorized user if necessary

        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        let user_found = UserAppEvent::find_one(
            &db,
            doc! {"_id": ObjectId::parse_str(event_id).expect("problem parsing id"), "userId": user_id},
            find_one_options,
        )
        .await
        .map_err(|_| Status::not_found("User event not found"))?;
        // .unwrap_or(Err(Status::not_found("user app event not found")));
        // .expect("Id not found");

        match user_found {
            Some(user) => {
                let id = user.id.expect("Problem getting user event id").to_string();
                // let k = ObjectId::parse_str(id);
                let user_app_event_response = UserAppEventResponse {
                    id,
                    user_id: user.user_id.to_string(),
                    event_name: user.event_name,
                    page: user.page,
                    description: user.description,
                };
                return Ok(Response::new(user_app_event_response));
            }
            None => return Err(Status::not_found("message")),
        }
    }

    async fn get_all_user_app_events(
        &self,
        request: tonic::Request<app_analytics::GetAllUserAppEventsRequest>,
    ) -> anyhow::Result<tonic::Response<app_analytics::GetAllUserAppEventsResponse>, tonic::Status>
    {
        let GetAllUserAppEventsRequest { user_id } = request.into_inner();

        let db = establish_connection().await;

        let user_app_event_found = UserAppEvent::find(&db, doc! {"userId": user_id}, None)
            .await
            .map_err(|_| Status::not_found("User data not found"))
            .expect("Problem finding user");

        let user_app_event = model_cursor_to_vec(user_app_event_found)
            .await
            .map_err(|_| Status::aborted("problem converting"))?
            .into_iter()
            .map(|event| {
                let id = event.id.expect("Problem getting event id").to_string();
                UserAppEventResponse {
                    id,
                    user_id: event.user_id.to_string(),
                    event_name: event.event_name,
                    page: event.page,
                    description: event.description,
                }
            })
            .collect::<Vec<UserAppEventResponse>>();

        Ok(Response::new(GetAllUserAppEventsResponse {
            user_app_event,
        }))
    }
}

pub struct AnalyticsApp;

impl AnalyticsApp {
    pub fn get_server() -> AppAnalyticsServer<AnalyticsService> {
        AppAnalyticsServer::new(AnalyticsService::default())
    }
}
