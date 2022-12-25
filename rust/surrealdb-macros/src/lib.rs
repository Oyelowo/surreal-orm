#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
pub use surrealdb_derive::FieldsGetter;

pub trait FieldsGetter {
    type Fields;
    fn get_fields_serialized() -> Self::Fields;
}

pub trait SurrealdbModel {
    type Fields;
    fn get_fields_serialized() -> Self::Fields;
}

// Examples

pub trait HelloMacro {
    fn hello_macro();
}
pub use surrealdb_derive::HelloMacro;

pub trait CollectionCrud {
    fn save();
}

pub trait MyTrait {
    fn answer() -> u32;
    fn level() -> &'static str;
}

pub use surrealdb_derive::MyTrait;
