 tree -I "target"      
.
├── Cargo.lock
├── Cargo.toml
├── backend-main
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── backend-second
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

Build backend-main
`cargo build -p backend-main`

Run backend-main alone
`cargo run -p backend-main`