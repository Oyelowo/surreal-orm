use crate::statements::select::TargettablesForSelect;
use crate::Binding;
use crate::Buildable;
use crate::SurrealSimpleId;
use crate::SurrealdbModel;
use crate::Valuex;

struct SurrealIdRange<T: SurrealdbModel> {
    start: SurrealSimpleId<T>,
    end: SurrealSimpleId<T>,
}

impl<T: SurrealdbModel> SurrealIdRange<T> {
    pub fn new(start: SurrealSimpleId<T>, end: SurrealSimpleId<T>) -> Self {
        SurrealIdRange { start, end }
    }
}

use std::ops::Bound;
use std::ops::RangeBounds;
use surrealdb::sql;
use surrealdb::sql::Thing;

struct UuidRange {
    start: Bound<Uuid>,
    end: Bound<Uuid>,
}
impl RangeBounds<Uuid> for UuidRange {
    fn start_bound(&self) -> Bound<&Uuid> {
        match &self.start {
            Bound::Included(uuid) => Bound::Included(uuid),
            Bound::Excluded(uuid) => Bound::Excluded(uuid),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&Uuid> {
        match &self.end {
            Bound::Included(uuid) => Bound::Included(uuid),
            Bound::Excluded(uuid) => Bound::Excluded(uuid),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    // fn is_inclusive(&self) -> bool {
    //     matches!(&self.start, Bound::Included(_) | Bound::Excluded(_))
    //         || matches!(&self.end, Bound::Included(_) | Bound::Excluded(_))
    // }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
struct Uuid(String);

impl std::ops::Deref for Uuid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<std::ops::RangeInclusive<Uuid>> for UuidRange {
    fn from(range: std::ops::RangeInclusive<Uuid>) -> Self {
        // let start = Bound::Included(range.start());
        //     let end = Bound::Included(range.end());
        //     UuidRange { start, end }
        todo!()
    }
}
trait Tana {}

//impl From Range for SurrealSimpleId<T>

impl<T> From<std::ops::RangeInclusive<SurrealSimpleId<T>>> for TargettablesForSelect
where
    T: SurrealdbModel,
{
    fn from(range: std::ops::RangeInclusive<SurrealSimpleId<T>>) -> Self {
        let start = range.start();
        let end = range.end();
        let table = T::table_name();
        let start_binding = Binding::new(start.to_thing().id).as_raw();
        let end_binding = Binding::new(end.to_thing().id).as_raw();
        let build = format!(
            "{table}:{}..{}",
            start_binding.get_param_dollarised(),
            end_binding.get_param_dollarised()
        );
        TargettablesForSelect::RecordRange(Valuex {
            string: build,
            bindings: vec![start_binding, end_binding],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{statements::select, All, Parametric, TestUser, ToRaw};

    use super::*;

    #[test]
    fn ere() {
        let id1 = TestUser::create_simple_id();
        let id2 = TestUser::create_simple_id();
        let statement = select(All).from(id1..=id2);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..$_param_00000002;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            statement.to_raw().build(),
            format!(
                "SELECT * FROM user:{}..{};",
                bindings[0].get_raw_value(),
                bindings[1].get_raw_value()
            )
        );

        let ref a = Uuid("a".to_string());
        let ref b = Uuid("b".to_string());
        let range = a..=b;
        // if range.start == a && range.end == b {
        //     println!("Range operator: ..=");
        // } else if range.start == a && range.end != b {
        //     println!("Range operator: ..");
        // } else if range.start != a && range.end == b {
        //     println!("Range operator: =..");
        // } else {
        //     println!("Range operator: ..?");
        // }
        //
        // let a = "a".to_string();
        // let b = "b".to_string();
        let range = a..;
        let range = a..b;
        let range = ..b;
        let range = ..=b;
        let range = a..=b;
        let range = ..;
        let range = ..;
        // range.start;
    }
}
