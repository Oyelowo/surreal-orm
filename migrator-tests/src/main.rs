use surreal_models::migrations::Resources;
use surreal_orm::migrator::Migrator;

#[tokio::main]
async fn main() {
    Migrator::run(Resources).await;
}
