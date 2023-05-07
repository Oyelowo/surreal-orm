use crate::statements::select::TargettablesForSelect;
use crate::Binding;
use crate::SurrealSimpleId;
use crate::SurrealdbModel;
use crate::Valuex;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
struct Uuid(String);

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

impl<T> From<std::ops::RangeFrom<SurrealSimpleId<T>>> for TargettablesForSelect
where
    T: SurrealdbModel,
{
    fn from(range: std::ops::RangeFrom<SurrealSimpleId<T>>) -> Self {
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

impl<T> From<std::ops::RangeTo<SurrealSimpleId<T>>> for TargettablesForSelect
where
    T: SurrealdbModel,
{
    fn from(range: std::ops::RangeTo<SurrealSimpleId<T>>) -> Self {
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

impl<T> From<std::ops::RangeToInclusive<SurrealSimpleId<T>>> for TargettablesForSelect
where
    T: SurrealdbModel,
{
    fn from(range: std::ops::RangeToInclusive<SurrealSimpleId<T>>) -> Self {
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

impl<T> From<std::ops::Range<SurrealSimpleId<T>>> for TargettablesForSelect
where
    T: SurrealdbModel,
{
    fn from(range: std::ops::Range<SurrealSimpleId<T>>) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::{statements::select, *};

    use super::*;

    #[test]
    fn test_range_inclusive_for_simple_surreal_id() {
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
        let range = a..;
        let range = a..b;
        let range = ..b;
        let range = ..=b;
        let range = a..=b;
        let range = ..;
        let range = ..;
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
}
