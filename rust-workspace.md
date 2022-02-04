 tree -I "target"      
.
├── Cargo.lock
├── Cargo.toml
├── graphql-mongo
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── grpc-mongo
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── common
    ├── Cargo.toml
    └── src
        └── lib.rs

6 directories, 8 files




From the root
Build all
`cargo build`

Build graphql-mongo
`cargo build -p graphql-mongo`

Run graphql-mongo alone
`cargo run -p graphql-mongo`