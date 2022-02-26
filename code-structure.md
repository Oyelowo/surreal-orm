❯ tree -I "node_modules"
.
├── LICENSE
├── README.md
├── organization.md
├── rust
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── graphql-mongo
│   │   ├── Cargo.toml
│   │   ├── gql_test.graphql
│   │   ├── src
│   │   │   ├── dataloaders
│   │   │   ├── lib.rs
│   │   │   ├── main.rs
│   │   │   ├── schema.rs
│   │   │   ├── starwar
│   │   │   │   ├── mod.rs
│   │   │   │   ├── model.rs
│   │   │   │   ├── mutation.rs
│   │   │   │   ├── query_droid.rs
│   │   │   │   ├── query_human.rs
│   │   │   │   ├── query_root.rs
│   │   │   │   ├── service.rs
│   │   │   │   ├── subscription.rs
│   │   │   │   └── type_gql.rs
│   │   │   ├── user
│   │   │   │   ├── mod.rs
│   │   │   │   ├── model.rs
│   │   │   │   ├── mutation_root.rs
│   │   │   │   ├── query_root.rs
│   │   │   │   ├── query_user.rs
│   │   │   │   └── type_gql.rs
│   │   │   └── utils
│   │   │       ├── config.rs
│   │   │       └── database.rs
│   │   └── target
│   │       └── rls
│   │           └── debug
│   │               ├── build
│   │               │   └── crossbeam-utils-d0a563f1bdc04c2c
│   │               │       ├── out
│   │               │       │   ├── probe2.ll
│   │               │       │   ├── probe3.ll
│   │               │       │   └── probe4.ll
│   │               │       ├── output
│   │               │       ├── root-output
│   │               │       └── stderr
│   │               └── deps
│   │                   ├── libsocket2-045297572854df4c.rmeta
│   │                   └── save-analysis
│   │                       └── libsocket2-045297572854df4c.json
│   ├── grpc-mongo
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── common
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── macros
│   │       │   ├── calculator.rs
│   │       │   ├── helpers.rs
│   │       │   └── mod.rs
│   │       ├── util.rs
│   │       ├── util_module_alternative
│   │       │   └── greeter_alt.rs
│   │       ├── util_module_alternative.rs
│   │       └── utils
│   │           ├── greet.rs
│   │           ├── maths.rs
│   │           └── mod.rs
│   └── examplequery.graphql
├── rust-workspace.md
├── typescript
│   ├── --workspace
│   ├── LICENSE.txt
│   ├── README.md
│   ├── lerna.json
│   ├── package-lock.json
│   ├── package.json
│   ├── packages
│   │   ├── frontend-main
│   │   │   ├── README.md
│   │   │   ├── next-env.d.ts
│   │   │   ├── next.config.js
│   │   │   ├── package-lock.json
│   │   │   ├── package.json
│   │   │   ├── pages
│   │   │   │   ├── _app.tsx
│   │   │   │   ├── api
│   │   │   │   │   └── hello.ts
│   │   │   │   └── index.tsx
│   │   │   ├── postcss.config.js
│   │   │   ├── public
│   │   │   │   ├── favicon.ico
│   │   │   │   └── vercel.svg
│   │   │   ├── styles
│   │   │   │   ├── Home.module.css
│   │   │   │   └── globals.css
│   │   │   ├── tailwind.config.js
│   │   │   └── tsconfig.json
│   │   └── libraries-core
│   │       ├── package.json
│   │       ├── src
│   │       │   ├── components
│   │       │   │   ├── CardTailWindExample.tsx
│   │       │   │   ├── HelloWorld.tsx
│   │       │   │   └── TextField.tsx
│   │       │   └── index.ts
│   │       └── tsconfig.json
│   ├── postcss.config.js
│   ├── renovate.json
│   ├── tsconfig.build.json
│   └── tsconfig.json
└── usful.md

32 directories, 82 files


Rust backend

❯ tree -I "target"         
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile.development
├── Dockerfile.production
├── Makefile
├── README.md
├── common
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       ├── macros
│       │   ├── calculator.rs
│       │   ├── helpers.rs
│       │   └── mod.rs
│       ├── util.rs
│       ├── util_module_alternative
│       │   └── greeter_alt.rs
│       ├── util_module_alternative.rs
│       └── utils
│           ├── export_data.rs
│           ├── greet.rs
│           ├── maths.rs
│           └── mod.rs
├── docker-compose.yml
├── examplequery.graphql
├── graphql-mongo
│   ├── Cargo.toml
│   ├── generated
│   │   └── schema.graphql
│   ├── gql_test.graphql
│   └── src
│       ├── app
│       │   ├── mod.rs
│       │   ├── post
│       │   │   ├── migration.rs
│       │   │   ├── mod.rs
│       │   │   ├── model.rs
│       │   │   ├── mutation_root.rs
│       │   │   └── query_root.rs
│       │   └── user
│       │       ├── mod.rs
│       │       ├── model.rs
│       │       ├── mutation_root.rs
│       │       └── query_root.rs
│       ├── bin
│       │   ├── graphql-generator.rs
│       │   ├── hello.rs
│       │   └── hello_shared_sdk.rs
│       ├── configs
│       │   ├── configuration.rs
│       │   ├── graphql.rs
│       │   ├── mod.rs
│       │   └── utils.rs
│       ├── lib.rs
│       ├── main.rs
│       ├── services
│       └── utils
├── graphql-postgres
│   ├── Cargo.toml
│   ├── Dockerfile.migrations
│   ├── Makefile
│   ├── README.md
│   ├── dbscripts
│   │   └── postgres
│   ├── docker-compose.yml
│   ├── env.sample
│   ├── gql_test.graphql
│   ├── migrations
│   │   ├── 20220201151946_setup.sql
│   │   ├── 20220201152218_create_users_table.sql
│   │   └── 20220201152224_create_posts_table.sql
│   ├── sqlx-data.json
│   ├── src
│   │   ├── app
│   │   │   ├── mod.rs
│   │   │   ├── post
│   │   │   │   ├── mod.rs
│   │   │   │   ├── model.rs
│   │   │   │   ├── mutation_root.rs
│   │   │   │   └── query_root.rs
│   │   │   └── user
│   │   │       ├── mod.rs
│   │   │       ├── model.rs
│   │   │       ├── mutation_root.rs
│   │   │       └── query_root.rs
│   │   ├── configs
│   │   │   ├── configuration.rs
│   │   │   ├── graphql.rs
│   │   │   └── mod.rs
│   │   ├── dataloaders
│   │   ├── main.rs
│   │   ├── services
│   │   ├── tests
│   │   │   └── health_check.rs
│   │   └── utils
│   └── update-sqlx-data.sh
├── grpc-mongo
│   ├── Cargo.toml
│   ├── Helloworld.md
│   ├── SQLx_Migration.md
│   ├── build.rs
│   ├── grpc_setting_started.md
│   ├── grpc_setting_started.readme
│   ├── grpccurl.sh
│   ├── protobuf
│   │   ├── app_analytics.proto
│   │   ├── helloworld.proto
│   │   └── music.proto
│   └── src
│       ├── app
│       │   ├── app_analytics
│       │   │   ├── mod.rs
│       │   │   ├── model.rs
│       │   │   └── service.rs
│       │   ├── greetings
│       │   │   ├── hello.rs
│       │   │   └── mod.rs
│       │   ├── mod.rs
│       │   └── music
│       │       ├── fan.rs
│       │       └── mod.rs
│       ├── bin
│       │   ├── analytics.rs
│       │   ├── client.rs
│       │   └── client_music.rs
│       ├── configs
│       │   ├── configuration.rs
│       │   ├── connection.rs
│       │   ├── mod.rs
│       │   └── utils.rs
│       └── main.rs
├── my-macros
│   ├── Cargo.toml
│   ├── derive
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── mongo-orm
│   │           ├── foo_bar.rs
│   │           ├── hello.rs
│   │           └── mod.rs
│   └── src
│       ├── lib.rs
│       └── main.rs
└── slim.report.json

42 directories, 103 files
