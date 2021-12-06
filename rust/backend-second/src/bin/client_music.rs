use anyhow::Result;
use music::{fan_client::FanClient, CreateMusicLoverRequest, Empty, GetMusicLoverRequest};
pub mod music {
    tonic::include_proto!("music");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = FanClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetMusicLoverRequest { id: 5 });

    let response = client.get_music_lover(request).await?;

    println!("RESPONSE MUSIC LOVER={:?}", response);

    //////////////////////////////////////////////////////////////
    let request = tonic::Request::new(Empty {});

    let response = client.get_all_music_lovers(request).await?;

    println!("\nRESPONSE ALL MUSIC LOVER={:?}", response);

    //////////////////////////////////////////////////////////////

    let request = tonic::Request::new(CreateMusicLoverRequest {
        name: "Rafael".to_string(),
        favorite_songs: vec!["Song1".to_string(), "Song2".to_string()],
    });

    let response = client.create_music_lover(request).await?;

    println!("\n CREATE NEW MUSIC LOVER={:?}", response);

    Ok(())
}
