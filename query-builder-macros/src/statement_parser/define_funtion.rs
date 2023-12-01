/// A macro to generate a function definition statement and the corresponding helper function.
/// e.g. `define_function!(get_it(first: bool, last: string, birthday: string) { let person = "43"; return person; });`
/// generates a `get_it_statement` itself and `get_it` helper function created by the macro.
///
/// # Arguments
/// * `function definition` - The function definition
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::define_function};
///
/// // The below generates a `get_it_statement` itself and `get_it` helper function created by the macro.
/// define_function!(get_it(first: bool, last: string, birthday: string) {
///     let person = "43";
///     return person;
/// });
/// ```
///
/// ```rust, ignore   
/// // The below generates a `get_person_statement` itself and `get_person` helper function created by the macro.
/// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
///     let person = select(All)
///         .from(SpaceShip::table_name())
///         .where_(
///             cond(SpaceShip::schema().id.equal(&first_arg))
///                 .and(SpaceShip::schema().name.equal(&last_arg))
///                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
///         );
///
///     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
///                 .then(person.with_path::<SpaceShip>(index(0)))
///             .else_(
///                 create::<SpaceShip>(
///                     vec![
///                         SpaceShip::schema().id.equal_to(&first_arg),
///                         SpaceShip::schema().name.equal_to(&last_arg),
///                         SpaceShip::schema().created.equal_to(&birthday_arg),
///                     ]
///                 )
///             ).end();
/// });
/// ```
#[macro_export]
macro_rules! define_function_ {
    ($function_name:ident ($($param:ident : $type:ident),* ) {$(let $var:ident = $value:expr;)* return $expr:expr;}) => {
        macro_rules! check_field_type {
            (any) => {
                $crate::ValueLike
            };
            (array) => {
                $crate::ArrayLike
            };
            (bool) => {
                $crate::BoolLike
            };
            (datetime) => {
                $crate::DatetimeLike
            };
            (string) => {
                $crate::StrandLike
            };
            (number) => {
                $crate::NumberLike
            };
            (int) => {
                $crate::NumberLike
            };
            (float) => {
                $crate::NumberLike
            };
            (decimal) => {
                $crate::NumberLike
            };
            (duration) => {
                $crate::DurationLike
            };
            (object) => {
                $crate::ObjectLike
            };
            (record) => {
                $crate::ThingLike
            };
            (geometry) => {
                $crate::GeometryLike
            };
            ($field_type:expr) => {{
                compile_error!(concat!("Invalid field type: ", $field_type));
                unreachable!();
            }};
        }
        $crate::internal_tools::paste! {
            pub fn [<$function_name _statement>]() -> $crate::statements::DefineFunctionStatement{
                {
                    $(
                        let $param = $crate::Param::new(stringify!($param));
                        // let field_type: $crate::FieldType = stringify!($type).parse().unwrap();

                    )*

                    $(
                        let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
                    )*

                    let body = $crate::
                    $(
                        chain(&$var).
                    )*

                    chain($crate::statements::return_($expr)).as_block();

                    $crate::statements::define_function(stringify!($function_name))
                        .params(vec![$(($param, stringify!($type).to_string())),*])
                        .body(body)
                }
            }

            pub fn [<$function_name>]($($param: impl Into<check_field_type!($type)>),*) -> $crate::Function {
                use $crate::Buildable as _;
                use $crate::Parametric as _;
                {
                    let mut __private_bindings = vec![];
                    let mut __private_args = vec![];
                    $(
                        let $param: check_field_type!($type) = $param.into();
                        __private_bindings.extend($param.get_bindings());
                        __private_args.push($param.build());
                    )*

                let build = format!("{}({})", stringify!(#function_name), __private_args.join(", "));
                $crate::Function::new()
                    .with_args_string(build)
                    .with_bindings(__private_bindings)
                }

            }
        }
    };
}

pub use define_function_ as define_function;
