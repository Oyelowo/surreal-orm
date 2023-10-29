// use crate::FileManager;
//
// pub async fn run_against_db(
//     &self,
//     all_migrations: Vec<impl Into<MigrationOneWay>>,
// ) -> MigrationResult<()> {
//     let migration::Schema {
//         name, timestamp, ..
//     } = &Migration::schema();
//     let migration_table = Migration::table_name();
//
//     // Get the latest migration
//     let latest_migration = select(All)
//         .from(migration_table.clone())
//         .order_by(timestamp.desc())
//         .limit(1)
//         .return_one::<Migration>(self.db())
//         .await?;
//
//     // Get migrations that are not yet applied
//     let migrations_to_run = all_migrations
//         .into_iter()
//         .map(|m| {
//             let m: MigrationOneWay = m.into();
//             m
//         })
//         .filter(|m| {
//             latest_migration.as_ref().map_or(true, |latest_migration| {
//                 m.timestamp > latest_migration.timestamp
//             })
//         })
//         .collect::<Vec<_>>();
//
//     // Get queries to run
//     let migration_queries = migrations_to_run
//         .iter()
//         .map(|m| m.content.clone())
//         .collect::<Vec<_>>()
//         .join("\n");
//
//     // Create queries to mark migrations as applied
//     let mark_queries_registered_queries = migrations_to_run
//         .iter()
//         .map(|m| Migration::create_raw(m.id.clone(), m.name.clone(), m.timestamp).build())
//         .collect::<Vec<_>>()
//         .join("\n");
//
//     println!("Running queries: {}", migration_queries);
//
//     // Join migrations with mark queries
//     let all = format!("{}\n{}", migration_queries, mark_queries_registered_queries);
//
//     // Run them as a transaction against a local in-memory database
//     if !all.trim().is_empty() {
//         begin_transaction()
//             .query(Raw::new(all))
//             .commit_transaction()
//             .run(self.db())
//             .await?;
//     }
//
//     Ok(())
// }
//
// async fn run_pending_migration(file_manager: FileManager) {
//     let all_migrations = file_manager.get_oneway_migrations(false).unwrap();
// }
//
// fn get_oneway_migration() {
//     let all_migrations = file_manager.get_oneway_migrations(false).unwrap();
// }
//
// async fn run_pending_embedded_migration(file_manager: FileManager) {
//     let all_migrations = file_manager.get_oneway_migrations(false).unwrap();
// }
// async fn rollback_migrations(file_manager: FileManager) {}
