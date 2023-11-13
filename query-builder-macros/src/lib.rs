mod query;
mod query_builder;

#[proc_macro]
pub fn query_raw(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let input = input.value();
    let sql = sql::parse(input.as_str());

    match sql {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };
    raw_input
}

#[proc_macro]
pub fn query(raw_input: TokenStream) -> TokenStream {
    query::query(raw_input.into()).into()
}

#[proc_macro]
pub fn query_turbo(input: TokenStream) -> TokenStream {
    query_builder::query_turbo(input.into()).into()
}

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    query_builder::query_block(input.into()).into()
}

#[proc_macro]
pub fn transaction(input: TokenStream) -> TokenStream {
    query_builder::query_transaction(input.into()).into()
}

/// A helper function to create a for loop
/// ```
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{for_, select, select_value}};
///
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
///
/// for_!((name in vec!["Oyelowo", "Oyedayo"]) {
///    select(All).from(person_table).where_(user_name.eq(name));
///    select(All).from(person_table).where_(user_name.eq(name));
///
///    for_!((name in select_value(user_name).from_only(person_table)) {
///         select(All).from(person_table).where_(user_name.eq(name));
///         select(All).from(person_table).where_(user_name.eq(name));
///    });
/// });
/// ```
#[macro_export]
macro_rules! for_loop {
    (($param:ident in $iterable:expr) { $($stmt:expr;)+ }) => {{
        let ref $param = $crate::Param::new(stringify!($param));
        $crate::statements::for_($param).in_($iterable).block($crate::block! {
            $($stmt;)+
        })
    }};
    (($param:ident IN $iterable:expr) { $($stmt:expr;)+ }) => {{
        let ref $param = $crate::Param::new(stringify!($param));
        $crate::statements::for_($param).in_($iterable).block($crate::block! {
            $($stmt;)+
        })
    }};
}
pub use for_loop as for_;
