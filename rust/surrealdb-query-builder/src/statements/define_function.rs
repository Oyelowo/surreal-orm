/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{BindingsList, Block, Buildable, Erroneous, ErrorList, Param, Parametric, Queryable};
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

/// A function definition statement
#[derive(Debug, Clone)]
pub struct DefineFunctionStatement {
    name: String,
    params: Vec<(Param, ParamType)>,
    body: Option<Block>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineFunctionStatement {
    /// Create a new function definition statement
    ///
    /// # Arguments
    /// * `name` - The name of the function
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
            body: None,
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Sets the parameters for the function
    pub fn params(mut self, params: Vec<(Param, ParamType)>) -> Self {
        self.params = params;
        self
    }

    /// Sets the body of the function
    pub fn body(mut self, body: Block) -> Self {
        self.bindings.extend(body.get_bindings());
        self.errors.extend(body.get_errors());
        self.body = Some(body);
        self
    }
}

/// Create a new function definition statement
pub fn define_function(name: impl Into<String>) -> DefineFunctionStatement {
    DefineFunctionStatement {
        name: name.into(),
        params: vec![],
        body: None,
        bindings: vec![],
        errors: vec![],
    }
}

impl Parametric for DefineFunctionStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for DefineFunctionStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Queryable for DefineFunctionStatement {}

impl Buildable for DefineFunctionStatement {
    fn build(&self) -> String {
        let mut build = format!("DEFINE FUNCTION {}(", self.name);
        build.push_str(
            &self
                .params
                .iter()
                .map(|(param, param_type)| format!("{}: {}", param.build(), param_type))
                .collect::<Vec<String>>()
                .join(", "),
        );
        build.push_str(") ");
        if let Some(body) = &self.body {
            build.push_str(&body.build());
        }
        format!("{build};")
    }
}

/// Define a function
#[macro_export]
macro_rules! define_function_ {
    ($function_name:ident ($($param:ident : $type:ident),* ) {$(let $var:ident = $value:expr;)* return $expr:expr;}) => {
        ::paste::paste! {
            pub fn [<$function_name _statement>]() -> DefineFunctionStatement{
                {
                    $(
                        let $param = $crate::Param::new(stringify!($param));
                        // let field_type: $crate::FieldType = stringify!($type).parse().unwrap();

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
                let build = format!("{}({})", stringify!($function_name), __private_args.join(", "));
                $crate::Function::new()
                    .with_args_string(build)
                    .with_bindings(__private_bindings)
                }

            }

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

macro_rules! check_field_type {
    (any) => {
        $crate::Valuex
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

fn ere() {
    define_function!(get_it(first: bool, last: string, birthday: string) {
        let person = "43";
        return person;
    });
    let xx = get_it(false, "3".to_string(), "3".to_string());

    let xx = get_it_statement();
}

#[cfg(test)]
mod tests {
    use crate::ToRaw;

    use super::*;

    define_function!(get_it(first: bool, last: string, birthday: string) {
        let person = "43";
        return person;
    });

    #[test]
    fn test_define_function() {
        let fn_statement = get_it_statement();

        insta::assert_display_snapshot!(fn_statement.to_raw().build());
        insta::assert_display_snapshot!(fn_statement.fine_tune_params());
        assert_eq!(
            fn_statement.to_raw().build(),
            "DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {\n\
                LET $person = '43';\n\n\
                RETURN $person;\n\
                };"
        );
        assert_eq!(
            fn_statement.fine_tune_params(),
            "DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {\n\
            LET $person = $_param_00000001;\n\n\
            RETURN $person;\n\
            };"
        );
        let x = get_it(false, "3".to_string(), "3".to_string());
    }
}
