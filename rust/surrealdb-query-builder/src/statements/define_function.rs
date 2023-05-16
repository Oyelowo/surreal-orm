// -- Define a global function which can be used in any query
// DEFINE FUNCTION fn::get_person($first: string, $last: string, $birthday: string) {
//
// 	LET $person = SELECT * FROM person WHERE [first, last, birthday] = [$first, $last, $birthday];
//
// 	RETURN IF $person[0].id THEN
// 		$person[0]
// 	ELSE
// 		CREATE person SET first = $first, last = $last, birthday = $birthday
// 	END;
//
// };
//
// -- Call the global custom function, receiving the returned result
// LET $person = fn::get_person('Tobie', 'Morgan Hitchcock', '2022-09-21');

type ParamType = String;

pub struct DefineFunctionStatement {
    name: String,
    params: Vec<(Param, ParamType)>,
    body: Option<Block>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineFunctionStatement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
            body: None,
            bindings: vec![],
            errors: vec![],
        }
    }

    pub fn params(mut self, params: Vec<(Param, ParamType)>) -> Self {
        self.params = params;
        self
    }

    pub fn body(mut self, body: Block) -> Self {
        self.bindings.extend(body.get_bindings());
        self.errors.extend(body.get_errors());
        self.body = Some(body);
        self
    }
}

pub fn define_function(name: impl Into<String>) -> DefineFunctionStatement {
    DefineFunctionStatement {
        name: name.into(),
        params: vec![],
        body: None,
        bindings: vec![],
        errors: vec![],
    }
}

// #[macro_export]
// macro_rules! define_function_ {
//     ($name:ident, $params:ident, $body:ident) => {
//         pub fn $name($params: Vec<String>, $body: impl Into<String>) -> DefineFunctionStatement {
//             define_function(stringify!($name), $params, $body)
//         }
//     };
// }
// ($($arg:expr),*), {$($code:tt)*}

#[macro_export]
macro_rules! define_function_ {
    ($function_name:ident ($($param:ident : $type:ident),* ) {$(let $var:ident = $value:expr;)* return $expr:expr;}) => {
        ::paste::paste! {
            pub fn [<$function_name _statement>]() -> DefineFunctionStatement{
                {
                    $(
                        let $param = $crate::Param::new(stringify!($param));
                        let field_type: $crate::FieldType = stringify!($type).parse().unwrap();

                    )*

                    $(
                        let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
                    )*

                    let body = $crate::statements::
                    $(
                        chain(&$var).
                    )*

                    chain($crate::statements::return_($expr)).as_block();

                    $crate::statements::define_function(stringify!($function_name))
                        .params(vec![$(($param, stringify!($type).to_string())),*])
                        .body(body)
                }
            }

            pub fn [<$function_name _fn>]($($param: impl Into<check_field_type!($type)>),*) {}

            // https://github.com/rust-lang/rust/issues/35853
            // macro_rules! $function_name {
            //     (@inner $($xx:expr),*) => {
            //         // [<$function_name _statement>]().body.unwrap().build()
            //     };
            // }
        }
    };
}
pub use define_function_ as define_function;
use quote::format_ident;
macro_rules! check_field_type {
    (any) => {
        $crate::Valuex
    };
    (array) => {
        $crate::ArrayLike
    };
    // (array) => { FieldType::Array };
    // (bool) => { FieldType::Bool };
    // (datetime) => { FieldType::DateTime };
    // (decimal) => { FieldType::Decimal };
    // (duration) => { FieldType::Duration };
    // (float) => { FieldType::Float };
    // (int) => { FieldType::Int };
    // (number) => { FieldType::Number };
    // (object) => { FieldType::Object };
    // (string) => { FieldType::String };
    (string) => {
        $crate::StrandLike
    };
    // (record()) => { FieldType::RecordAny };
    // (geometry($($geom:ident),+)) => {{
    //     let geometries = vec![
    //         $(GeometryType::$geom),+
    //     ];
    //     FieldType::Geometry(geometries)
    // }};
    ($field_type:expr) => {{
        compile_error!(concat!("Invalid field type: ", $field_type));
        unreachable!();
    }};
}

fn erer(mm: impl Into<check_field_type!(any)>) {}
fn ere() {
    define_function!(get_it(first: string, last: string, birthday: string) {
        let person = "43";
        return person;
    });
    get_it_fn("3".to_string(), "3".to_string(), "3".to_string());
    // get_it!(first, last, birthday);

    // get_it_statement()
}

use crate::{BindingsList, Block, Erroneous, ErrorList, Param, Parametric};
