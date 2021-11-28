use anyhow::{Context, Result};


fn main()-> Result<()>  {
    let files = &["helloworld/helloworld.proto", "helloworld/music_lovers.proto"];
    let dirs = &["../protobuf"];

    tonic_build::configure().build_server(true).build_client(true)
    .out_dir("./src/grpc_generated_proto")
    .compile(files, dirs)
    .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

       // recompile protobufs only if any of the proto files changes.
    for file in files {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}





/* 
use std::env;
use std::path::PathBuf;

fn main() {
    tonic_build::configure()
        .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    tonic_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .server_attribute("Echo", "#[derive(PartialEq)]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .client_attribute("Echo", "#[derive(PartialEq)]")
        .compile(&["proto/attrs/attrs.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .compile(
            &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}

*/