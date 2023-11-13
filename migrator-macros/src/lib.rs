mod migration;
use migration::generate_embedded_migrations;
use proc_macro::TokenStream;

/// embed_migrations!() is a macro that embeds migrations in the binary at compile time.
/// It takes 3 arguments:
/// 1. The path to the migrations directory
/// 2. The migration flag
/// 3. The migration mode
/// The path to the migrations directory is optional. If not provided, it defaults to 'migrations'.
/// The migration flag is optional. If not provided, it defaults to MigrationFlag::TwoWay.
/// The migration mode is optional. If not provided, it defaults to Mode::Strict.
///
/// # Example
/// ```
/// use surreal_orm::migrator::{self, embed_migrations};
/// use surreal_orm::migrator::{FileManager, MigrationFlag, MigratorDatabase, Mode};
///
/// // Embed migrations as constant
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!();
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations");
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations", one_way);
/// const MIGRATIONS_ONE_WAY: migrator::EmbeddedMigrationsOneWay = embed_migrations!("migrations", one_way, strict);
///
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!();
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations");
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations", two_way);
/// const MIGRATIONS_TWO_WAY: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("migrations", two_way, strict);
/// ```
#[proc_macro]
pub fn embed_migrations(input: TokenStream) -> TokenStream {
    generate_embedded_migrations(input)
}
