use crate::{Field, Model, Raw, Table};

/// Provides the ability to gather all the resources
/// of a table in a single place. Fields definitions,
/// and table definitions are using automatically
/// generated methods from the `Model` trait.
/// Events and indexes definitions are manually
/// implemented using the statements or query macro
/// provided by the crate.
pub trait TableResources
where
    Self: Model,
{
    /// Returns a list of fields definitions.
    fn events_definitions() -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields' indexes definitions.
    fn indexes_definitions() -> Vec<Raw> {
        vec![]
    }

    /// Returns a list of fields definitions.
    fn fields_definitions() -> Vec<Raw> {
        Self::define_fields()
    }

    /// Returns a table definition.
    fn table_definition() -> Raw {
        Self::define_table()
    }
}

/// A trait for field changes.
#[derive(Debug, Clone)]
pub struct FieldChangeMeta {
    /// The table where the field is located.
    pub table: Table,
    /// The old name of the field.
    pub old_name: Field,
    /// The new name of the field.
    pub new_name: Field,
}

/// A trait for table changes.
#[macro_export]
macro_rules! create_table_resources {
    ($($struct_table: ident),*) => {
        fn tables(&self) -> ::std::vec::Vec<$crate::Raw> {
            ::std::vec![
                $(
                    ::std::vec![<$struct_table as $crate::TableResources>::table_definition()],
                    <$struct_table as $crate::TableResources>::fields_definitions(),
                    <$struct_table as $crate::TableResources>::indexes_definitions(),
                    <$struct_table as $crate::TableResources>::events_definitions(),
                )*
            ].into_iter().flatten().collect::<::std::vec::Vec<$crate::Raw>>()
        }


        fn tables_fields_meta(&self) -> ::std::collections::HashMap<$crate::Table, ::std::vec::Vec<$crate::FieldMetadata>> {
            let mut meta = ::std::collections::HashMap::<$crate::Table, ::std::vec::Vec<$crate::FieldMetadata>>::new();
            $(
                meta.insert(<$struct_table as $crate::Model>::table(), <$struct_table as $crate::Model>::get_field_meta());
            )*
            meta
        }


    };
}
