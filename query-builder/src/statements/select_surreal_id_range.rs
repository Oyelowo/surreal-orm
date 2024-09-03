/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::ops::Bound;

use crate::{
    statements::select::TargettablesForSelect, Binding, Model, SurrealId, SurrealSimpleId,
    SurrealUlid, SurrealUuid, ValueLike,
};
use surrealdb::sql;

impl<T, V> From<std::ops::RangeInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeInclusive<SurrealId<T, V>>) -> Self {
        // e.g user:1..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start().to_thing().id),
            end: Bound::Included(range.end().to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeFrom<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeFrom<SurrealId<T, V>>) -> Self {
        // e.g user:1..
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Unbounded,
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeTo<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeTo<SurrealId<T, V>>) -> Self {
        // e.g user:..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeToInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeToInclusive<SurrealId<T, V>>) -> Self {
        // e.g user:..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Included(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::Range<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::Range<SurrealId<T, V>>) -> Self {
        // e.g user:1..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeInclusive<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeInclusive<&SurrealId<T, V>>) -> Self {
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start().to_thing().id),
            end: Bound::Included(range.end().to_thing().id),
        };
        let binding = Binding::new(range);

        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeFrom<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeFrom<&SurrealId<T, V>>) -> Self {
        // e.g user:1..
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Unbounded,
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeTo<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeTo<&SurrealId<T, V>>) -> Self {
        // e.g user:..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeToInclusive<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeToInclusive<&SurrealId<T, V>>) -> Self {
        // e.g user:..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Included(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::Range<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::Range<&SurrealId<T, V>>) -> Self {
        // e.g user:1..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

//////////////////////////////////////////
macro_rules! create_range {
    ($id_type:ident) => {
        impl<T> From<std::ops::RangeInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeInclusive<$id_type<T>>) -> Self {
                // e.g user:1..=5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start().to_thing().id),
                    end: Bound::Included(range.end().to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeFrom<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeFrom<$id_type<T>>) -> Self {
                // e.g user:1..
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start.to_thing().id),
                    end: Bound::Unbounded,
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeTo<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeTo<$id_type<T>>) -> Self {
                // e.g user:..5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Unbounded,
                    end: Bound::Excluded(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeToInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeToInclusive<$id_type<T>>) -> Self {
                // e.g user:..=5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Unbounded,
                    end: Bound::Included(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::Range<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::Range<$id_type<T>>) -> Self {
                // e.g user:1..5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start.to_thing().id),
                    end: Bound::Excluded(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }
    };
}

create_range!(SurrealSimpleId);
create_range!(SurrealUuid);
create_range!(SurrealUlid);
///////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{statements::select, *};

    macro_rules! gen_test {
        ($range:expr) => {
            let statement = select(All).from($range);
            assert_eq!(
                statement.fine_tune_params(),
                "SELECT * FROM $_param_00000001;"
            );
            let bindings = statement.get_bindings();
            assert_eq!(bindings.len(), 1);
            assert_eq!(
                statement.to_raw().build(),
                format!("SELECT * FROM {};", bindings[0].get_raw_value())
            );
        };
    }

    #[test]
    fn test_range_from_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let id2 = TestUser::create_simple_id();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let id2 = TestUser::create_id("oyedayo");
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let id2 = TestUser::create_id("oyedayo");
        gen_test!(id1..id2);
    }
}
