/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_models::migrations::Resources;
use surreal_orm::migrator::Migrator;

#[tokio::main]
async fn main() {
    Migrator::run(Resources).await;
}
