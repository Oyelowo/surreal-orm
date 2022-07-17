#[macro_export]
macro_rules! sync_mongo_models {
    ($db:expr; $($model:ident),*) => {
        use anyhow::Context;
        /*
        Does this for all the models
         User::sync(db)
        .await
        .with_context(|| "Problem syncing users")?;
         */
       $( $model::sync($db).await.with_context(|| format!("Problem syncing {}", $model::COLLECTION_NAME))?;)*
    };
}
