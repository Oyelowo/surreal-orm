use anyhow::Result;
use app_analytics::{
    app_analytics_client::AppAnalyticsClient, CreateUserAppEventRequest,
    GetAllUserAppEventsRequest, GetAllUserAppEventsResponse, GetUserAppEventRequest,
};
pub mod app_analytics {
    tonic::include_proto!("app_analytics");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = AppAnalyticsClient::connect("http://[::1]:50051").await?;

    let response_create = client
        .create_user_app_event(tonic::Request::new(CreateUserAppEventRequest {
            user_id: "1".into(),
            event_name: "Open Chart".into(),
            page: "/charts".into(),
            description: "dashboard page".into(),
        }))
        .await?;

    println!("\n CREATE NEW USER EVENT={:?}", response_create);

    //////////////////////////////////////////////////////////////

    let response_get_all = client
        .get_all_user_app_events(tonic::Request::new(GetAllUserAppEventsRequest {
            user_id: "1".into(),
        }))
        .await?;

    println!("\nRESPONSE ALL USER APP EVENT={:?}", response_get_all);

    ////////////////////////////////////////////////////////////

    let get_one = client
        .get_user_app_event(tonic::Request::new(GetUserAppEventRequest {
            user_id: "1".into(),
            event_id: response_create.into_inner().id,
        }))
        .await?;

    println!("RESPONSE USER APP EVENT={:?}", get_one);
    Ok(())
}
