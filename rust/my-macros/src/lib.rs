pub trait HelloMacro {
    fn hello_macro();
}
pub use derive::HelloMacro;

pub trait CollectionCrud {
    fn save();
}

pub trait MyTrait {
    fn answer() -> u32;
    fn level() -> &'static str;
}

pub use derive::MyTrait;

pub trait FieldsGetter {
    type Fields;
    fn get_fields_serialized() -> Self::Fields;
}

pub use derive::FieldsGetter;
