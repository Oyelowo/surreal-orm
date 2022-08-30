use mongodb::bson::Bson;

/// Index sort order (useful for compound indexes).
///
/// [Mongo manual](https://docs.mongodb.com/manual/core/index-compound/#sort-order)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl From<SortOrder> for Bson {
    fn from(v: SortOrder) -> Self {
        match v {
            SortOrder::Ascending => Self::Int32(1),
            SortOrder::Descending => Self::Int32(-1),
        }
    }
}
