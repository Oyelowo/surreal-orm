use anyhow::{Result};
use tonic::{Request, Response, Status};
pub mod music_lovers {
    tonic::include_proto!("music_lovers");
}

use music_lovers::{
    lovers_server::{Lovers, LoversServer},
    CreateMusicLoverRequest, Empty, GetAllMusicLoversReply, GetMusicLoverRequest, MusicLoverReply,
};

#[derive(Debug, Default)]
pub struct MyMusicLovers {}

#[tonic::async_trait]
impl Lovers for MyMusicLovers {
    async fn create_music_lover(
        &self,
        request: Request<CreateMusicLoverRequest>,
    ) -> Result<Response<MusicLoverReply>, Status> {
        println!("create_music_lover: {:?}", request);
        let all_music_lovers = get_fake_music_lovers();
        let CreateMusicLoverRequest {
            name,
            favorite_songs,
        } = request.into_inner();

        Ok(Response::new(MusicLoverReply {
            id: all_music_lovers.len() as u32 + 1,
            name: name,
            favorite_songs: favorite_songs,
            message: "Fakely just created".to_string(),
        }))
    }

    async fn get_music_lover(
        &self,
        request: Request<GetMusicLoverRequest>,
    ) -> Result<Response<MusicLoverReply>, Status> {
        println!("Got a request: {:?}", request);

        let all_music_lovers = get_fake_music_lovers();
        let fallback_music_lover = MusicLoverReply {
            id: 100,
            name: "Backup user when not found".to_string(),
            favorite_songs: vec!["speechless".to_string(), "earthsong".to_string()],
            message: "".to_string(),
        };

        let found_music_lover = all_music_lovers
            .into_iter()
            .find(|lover| lover.id == request.get_ref().id as u32)
            .unwrap_or(fallback_music_lover);

        Ok(Response::new(found_music_lover))
    }

    async fn get_all_music_lovers(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<GetAllMusicLoversReply>, Status> {
        println!("Got a request: {:?}", request);
        let all_music_lovers = get_fake_music_lovers();

        let reply = GetAllMusicLoversReply {
             music_lovers: all_music_lovers
        };
        Ok(Response::new(reply))
    }
}

fn get_fake_music_lovers() -> Vec<MusicLoverReply> {
    let music_lovers = vec![
        MusicLoverReply {
            id: 1,
            name: "Oyelowo".to_string(),
            favorite_songs: vec!["speechless".to_string(), "earthsong".to_string()],
            message: "The first".to_string(),
        },
        MusicLoverReply {
            id: 2,
            name: "Oyedayo".to_string(),
            favorite_songs: vec!["another".to_string(), "Sandy".to_string()],
            message: "The second".to_string(),
        },
        MusicLoverReply {
            id: 3,
            name: "Samuel".to_string(),
            favorite_songs: vec!["Clayey".to_owned(), "Mars".to_string()],
            message: "The third".to_string(),
        },
        MusicLoverReply {
            id: 4,
            name: "Jukka".to_string(),
            favorite_songs: vec!["Grenada".to_string(), "Helsinki".to_string()],
            message: "The fourth".to_string(),
        },
        MusicLoverReply {
            id: 5,
            name: "MariKoi".to_string(),
            favorite_songs: vec!["Espoo".to_string(), "Canada".to_string()],
            message: "The fifth".to_string(),
        },
    ];
    music_lovers
}


pub struct MusicLoverApp {}

impl MusicLoverApp {
    pub fn new() -> LoversServer<MyMusicLovers> {
        LoversServer::new(MyMusicLovers::default())
    }
}
