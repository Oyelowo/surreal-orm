/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    statements::select::TargettablesForSelect, Binding, SurrealId, SurrealSimpleId, SurrealUlid,
    SurrealUuid, SurrealdbModel, Valuex,
};
use surrealdb::sql;

impl<T, V> From<std::ops::RangeInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeInclusive<SurrealId<T, V>>) -> Self {
        let start = range.start();
        let end = range.end();
        let table = T::table_name();
        let start_binding = Binding::new(start.to_thing().id).as_raw();
        let end_binding = Binding::new(end.to_thing().id).as_raw();
        let build = format!(
            "{table}:{}..={}",
            start_binding.get_param_dollarised(),
            end_binding.get_param_dollarised()
        );
        TargettablesForSelect::RecordRange(Valuex {
            string: build,
            bindings: vec![start_binding, end_binding],
        })
    }
}

impl<T, V> From<std::ops::RangeFrom<SurrealId<T, V>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeFrom<SurrealId<T, V>>) -> Self {
        let start = range.start;
        let table = T::table_name();
        let start_binding = Binding::new(start.to_thing().id).as_raw();
        let build = format!("{table}:{}..", start_binding.get_param_dollarised());
        TargettablesForSelect::RecordRange(Valuex {
            string: build,
            bindings: vec![start_binding],
        })
    }
}

impl<T, V> From<std::ops::RangeTo<SurrealId<T, V>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeTo<SurrealId<T, V>>) -> Self {
        let end = range.end;
        let table = T::table_name();
        let end_binding = Binding::new(end.to_thing().id).as_raw();
        let build = format!("{table}:..{}", end_binding.get_param_dollarised());
        TargettablesForSelect::RecordRange(Valuex {
            string: build,
            bindings: vec![end_binding],
        })
    }
}

impl<T, V> From<std::ops::RangeToInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeToInclusive<SurrealId<T, V>>) -> Self {
        let end = range.end;
        let table = T::table_name();
        let end_binding = Binding::new(end.to_thing().id).as_raw();
        let build = format!("{table}:..={}", end_binding.get_param_dollarised());
        TargettablesForSelect::RecordRange(Valuex {
            string: build,
            bindings: vec![end_binding],
        })
    }
}

impl<T, V> From<std::ops::Range<SurrealId<T, V>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::Range<SurrealId<T, V>>) -> Self {
        let start = range.start;
        let end = range.end;
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

//////////////////////////////////////////
macro_rules! create_range {
    ($id_type:ident) => {
        impl<T> From<std::ops::RangeInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: SurrealdbModel,
        {
            fn from(range: std::ops::RangeInclusive<$id_type<T>>) -> Self {
                let start = range.start();
                let end = range.end();
                let table = T::table_name();
                let start_binding = Binding::new(start.to_thing().id).as_raw();
                let end_binding = Binding::new(end.to_thing().id).as_raw();
                let build = format!(
                    "{table}:{}..={}",
                    start_binding.get_param_dollarised(),
                    end_binding.get_param_dollarised()
                );
                TargettablesForSelect::RecordRange(Valuex {
                    string: build,
                    bindings: vec![start_binding, end_binding],
                })
            }
        }

        impl<T> From<std::ops::RangeFrom<$id_type<T>>> for TargettablesForSelect
        where
            T: SurrealdbModel,
        {
            fn from(range: std::ops::RangeFrom<$id_type<T>>) -> Self {
                let start = range.start;
                let table = T::table_name();
                let start_binding = Binding::new(start.to_thing().id).as_raw();
                let build = format!("{table}:{}..", start_binding.get_param_dollarised());
                TargettablesForSelect::RecordRange(Valuex {
                    string: build,
                    bindings: vec![start_binding],
                })
            }
        }

        impl<T> From<std::ops::RangeTo<$id_type<T>>> for TargettablesForSelect
        where
            T: SurrealdbModel,
        {
            fn from(range: std::ops::RangeTo<$id_type<T>>) -> Self {
                let end = range.end;
                let table = T::table_name();
                let end_binding = Binding::new(end.to_thing().id).as_raw();
                let build = format!("{table}:..{}", end_binding.get_param_dollarised());
                TargettablesForSelect::RecordRange(Valuex {
                    string: build,
                    bindings: vec![end_binding],
                })
            }
        }

        impl<T> From<std::ops::RangeToInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: SurrealdbModel,
        {
            fn from(range: std::ops::RangeToInclusive<$id_type<T>>) -> Self {
                let end = range.end;
                let table = T::table_name();
                let end_binding = Binding::new(end.to_thing().id).as_raw();
                let build = format!("{table}:..={}", end_binding.get_param_dollarised());
                TargettablesForSelect::RecordRange(Valuex {
                    string: build,
                    bindings: vec![end_binding],
                })
            }
        }

        impl<T> From<std::ops::Range<$id_type<T>>> for TargettablesForSelect
        where
            T: SurrealdbModel,
        {
            fn from(range: std::ops::Range<$id_type<T>>) -> Self {
                let start = range.start;
                let end = range.end;
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
    };
}

create_range!(SurrealSimpleId);
create_range!(SurrealUuid);
create_range!(SurrealUlid);
///////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{statements::select, *};

    #[test]
    fn test_range_inclusive_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let id2 = TestUser::create_simple_id();
        let statement = select(All).from(id1..=id2);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..=$_param_00000002;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            statement.to_raw().build(),
            format!(
                "SELECT * FROM user:{}..={};",
                bindings[0].get_raw_value(),
                bindings[1].get_raw_value()
            )
        );
    }

    #[test]
    fn test_range_from_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let statement = select(All).from(id1..);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:{}..;", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let statement = select(All).from(..id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..{};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_inclusive_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let statement = select(All).from(..=id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..=$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..={};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let id2 = TestUser::create_simple_id();
        let statement = select(All).from(id1..id2);
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
    }

    #[test]
    fn test_range_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        let statement = select(All).from(id1..=id2);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..=$_param_00000002;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            statement.to_raw().build(),
            format!(
                "SELECT * FROM user:{}..={};",
                bindings[0].get_raw_value(),
                bindings[1].get_raw_value()
            )
        );
    }

    #[test]
    fn test_range_from_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let statement = select(All).from(id1..);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:{}..;", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let statement = select(All).from(..id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..{};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let statement = select(All).from(..=id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..=$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..={};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        let statement = select(All).from(id1..id2);
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
    }

    #[test]
    fn test_range_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        let statement = select(All).from(id1..=id2);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..=$_param_00000002;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            statement.to_raw().build(),
            format!(
                "SELECT * FROM user:{}..={};",
                bindings[0].get_raw_value(),
                bindings[1].get_raw_value()
            )
        );
    }

    #[test]
    fn test_range_from_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let statement = select(All).from(id1..);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:{}..;", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let statement = select(All).from(..id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..{};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let statement = select(All).from(..=id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..=$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..={};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        let statement = select(All).from(id1..id2);
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
    }

    #[test]
    fn test_range_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let id2 = TestUser::create_id("oyedayo");
        let statement = select(All).from(id1..=id2);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..=$_param_00000002;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(
            statement.to_raw().build(),
            format!(
                "SELECT * FROM user:{}..={};",
                bindings[0].get_raw_value(),
                bindings[1].get_raw_value()
            )
        );
    }

    #[test]
    fn test_range_from_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let statement = select(All).from(id1..);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:$_param_00000001..;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:{}..;", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let statement = select(All).from(..id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..{};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let statement = select(All).from(..=id1);
        assert_eq!(
            statement.fine_tune_params(),
            "SELECT * FROM user:..=$_param_00000001;"
        );
        let bindings = statement.get_bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(
            statement.to_raw().build(),
            format!("SELECT * FROM user:..={};", bindings[0].get_raw_value(),)
        );
    }

    #[test]
    fn test_range_for_surreal_custom() {
        let id1 = TestUser::create_id("oyelowo");
        let id2 = TestUser::create_id("oyedayo");
        let statement = select(All).from(id1..id2);
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
    }

    #[test]
    fn test_range_for_thing() {
        let thing1 = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let thing2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));

        let id1 = TestUser::from_thing(thing1).unwrap();
        let id2 = TestUser::from_thing(thing2).unwrap();
        let statement = select(All).from(id1..id2);
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
    }
}
