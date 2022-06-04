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
    // fn name() -> String;
}

pub use derive::MyTrait;

pub trait SpaceTrait {
    // fn name() -> String;
    type Naam;
    fn get_field_names() -> Self::Naam;
}

pub use derive::SpaceTrait;
