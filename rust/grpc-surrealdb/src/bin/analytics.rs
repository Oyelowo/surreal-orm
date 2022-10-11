use anyhow::Result;
use app_analytics::{
    app_analytics_client::AppAnalyticsClient, CreateUserAppEventRequest, GetAllUserAppEventsRequest,
};

pub mod app_analytics {
    #![allow(clippy::derive_partial_eq_without_eq)]
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
        .await?
        .into_inner();

    println!("\n CREATE NEW USER EVENT={:?}", response_create);

    //////////////////////////////////////////////////////////////

    let response_get_all = client
        .get_all_user_app_events(tonic::Request::new(GetAllUserAppEventsRequest {
            user_id: response_create.user_id.clone(),
        }))
        .await?
        .into_inner();

    println!("\nRESPONSE ALL USER APP EVENT={:?}", response_get_all);

    ////////////////////////////////////////////////////////////

    // let get_one = client
    //     .get_user_app_event(tonic::Request::new(GetUserAppEventRequest {
    //         event_id: "61fda82206cb096659bd294b".into(),
    //         user_id: "1".into(),
    //     }))
    //     .await?
    //     .into_inner();

    // println!("RESPONSE USER APP EVENT={:?}", get_one);
    Ok(())
}
