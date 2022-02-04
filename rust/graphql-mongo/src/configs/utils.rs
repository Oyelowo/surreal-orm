use futures::stream::StreamExt;
use wither::{Model, ModelCursor};

pub async fn model_cursor_to_vec<T: Model>(mut cursor: ModelCursor<T>) -> anyhow::Result<Vec<T>> {
    let mut collections: Vec<T> = vec![];
    while let Some(collection) = cursor.next().await {
        collections.push(collection?);
    }
    Ok(collections)
}
