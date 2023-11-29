use std::fmt::Display;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{self, parse::Parse, Ident, Token};

use super::{helpers::generate_variable_name, if_else::Body};

// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
//                 .then(person)
//             .else_(
//                 create::<SpaceShip>().set(
//                     vec![
//                         SpaceShip::schema().id.equal_to(&first_arg),
//                         SpaceShip::schema().name.equal_to(&last_arg),
//                         SpaceShip::schema().created.equal_to(&birthday_arg),
//                     ]
//                 )
//             ).end();
// });
//
// #[test]
// fn test_function_definition() {
//     let person = get_person("Oyelowo", "Oyedayo", "2022-09-21");
//     insta::assert_display_snapshot!(person.to_raw().build());
//     insta::assert_display_snapshot!(person.fine_tune_params());
//     assert_eq!(
//         person.to_raw().build(),
//         "get_person('Oyelowo', 'Oyedayo', '2022-09-21')"
//     );
//     assert_eq!(
//         person.fine_tune_params(),
//         "get_person($_param_00000001, $_param_00000002, $_param_00000003)"
//     );
//
//     let person_statement = get_person_statement();
//     insta::assert_display_snapshot!(person_statement.to_raw().build());
//     insta::assert_display_snapshot!(person_statement.fine_tune_params());
//
//     assert_eq!(
//         person_statement.to_raw().build(),
//         "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
//             LET $person = (SELECT * FROM space_ship WHERE (id = $first_arg) AND (name = $last_arg) AND \
//             (created = $birthday_arg));\n\n\
//             RETURN IF $person[0].id != NONE THEN \
//             $person[0] \
//             ELSE \
//             (CREATE space_ship SET id = $first_arg, name = $last_arg, created = $birthday_arg) \
//             END;\n\
//             };"
//     );
//
//     assert_eq!(person_statement.fine_tune_params(),
//     "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
//             LET $person = $_param_00000001;\n\n\
//             RETURN $_param_00000002;\n\
//             };"
//     );
// }
//
//

// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
//                 .then(person)
//             .else_(
//                 create::<SpaceShip>().set(
//                     vec![
//                         SpaceShip::schema().id.equal_to(&first_arg),
//                         SpaceShip::schema().name.equal_to(&last_arg),
//                         SpaceShip::schema().created.equal_to(&birthday_arg),
//                     ]
//                 )
//             ).end();
// });
//
//
// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     if person.with_path::<SpaceShip>(index(0)).id.is_not(NONE) {
//         return person;
//     } else {
//         return create::<SpaceShip>().set(
//             vec![
//                 SpaceShip::schema().id.equal_to(&first_arg),
//                 SpaceShip::schema().name.equal_to(&last_arg),
//                 SpaceShip::schema().created.equal_to(&birthday_arg),
//             ]
//         );
//     };
// });
// //

struct FieldTypeParser(FieldType);

impl FieldTypeParser {
    pub fn to_lib_type(&self) -> proc_macro2::TokenStream {
        let crate_name = get_crate_name(false);
        let FieldTypeParser(type_) = self;
        match type_ {
            FieldType::Any => quote!(#crate_name::ValueLike),
            FieldType::Bool => quote!(#crate_name::BoolLike),
            FieldType::Datetime => quote!(#crate_name::DatetimeLike),
            FieldType::String => quote!(#crate_name::StrandLike),
            FieldType::Number => quote!(#crate_name::NumberLike),
            FieldType::Int => quote!(#crate_name::NumberLike),
            FieldType::Float => quote!(#crate_name::NumberLike),
            FieldType::Decimal => quote!(#crate_name::NumberLike),
            FieldType::Duration => quote!(#crate_name::DurationLike),
            FieldType::Object => quote!(#crate_name::ObjectLike),
            FieldType::Record(_) => quote!(#crate_name::ThingLike),
            FieldType::Array(_, _) => quote!(#crate_name::ArrayLike),
            FieldType::Geometry(_) => quote!(#crate_name::GeometryLike),
            FieldType::Null => quote!(#crate_name::NullLike),
            FieldType::Bytes => quote!(#crate_name::BytesLike),
            FieldType::Uuid => quote!(#crate_name::UuidLike),
            FieldType::Option(_) => {
                quote!(::std::option::Option<#crate_name::ValueLike>)
            }
            FieldType::Union(_) => quote!(#crate_name::ValueLike),
            FieldType::Set(_, _) => quote!(#crate_name::SetLike),
        }
    }
}

impl Display for FieldTypeParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FieldTypeParser(type_) = self;
        write!(f, "{}", type_)
    }
}

impl ToTokens for FieldTypeParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let FieldTypeParser(type_) = self;
        let type_ = type_.to_string();
        tokens.extend(quote::quote!(#type_));
    }
}

impl Parse for FieldTypeParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_ = input.to_string().trim().parse::<FieldType>();
        match type_ {
            Ok(type_) => Ok(Self(type_)),
            Err(_) => Err(syn::Error::new(
                Span::call_site(),
                "expected a valid field type",
            )),
        }
    }
}

struct Argument {
    name: Ident,
    type_: FieldTypeParser,
}

impl Parse for Argument {
    // e.g first_arg: string
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![:]>()?;
        let type_ = input.parse::<FieldTypeParser>()?;
        Ok(Self { name, type_ })
    }
}

struct DefineFunctionStatementParser {
    function_name: Ident,
    params: Vec<Argument>,
    body: Body,
    generated_ident: Ident,
}

impl Parse for DefineFunctionStatementParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function_name = input.parse::<Ident>()?;

        let args_content;
        let _ = syn::parenthesized!(args_content in input);
        let params =
            syn::punctuated::Punctuated::<Argument, Token![,]>::parse_terminated(&args_content)?;

        let body = input.parse::<Body>()?;

        Ok(Self {
            function_name,
            params: params.into_iter().collect(),
            body,
            generated_ident: generate_variable_name(),
        })
    }
}

impl DefineFunctionStatementParser {
    pub fn tokenize(&self) -> TokenStream {
        let Self {
            function_name,
            params: args,
            body,
            generated_ident,
        } = self;
        let crate_name = get_crate_name(false);

        let params_rendered = args.iter().map(|param| {
            let name = &param.name;
            quote!(
                let #name = #crate_name::Param::new(stringify!(#name));
            )
        });

        let args_used = args.iter().map(|param| {
            let name = &param.name.to_string();
            let type_ = &param.type_.to_string();
            quote!(
                #crate_name::FunctionArgument {
                     name: #name.into(),
                     type_: #type_.parse::<#crate_name::FieldType>().expect("Invalid field type.")
                }
            )
        }).collect::<Vec<_>>();

        let function_stmt_name = format_ident!("{function_name}_statement");

        let define_function_statement: proc_macro2::TokenStream = quote!(
            pub fn #function_stmt_name() -> #crate_name::statements::DefineFunctionStatement{
                #( #params_rendered )*

                #crate_name::statements::define_function(stringify!($function_name))
                .arguments(::std::vec![#(#args_used),*])
                .body(body)
            }
        );

        // get_user(count: Number);
        // get_user(56);
        // X get_user("wrong input");

        let function_params = args.iter().map(|param| {
            let name = &param.name;
            let type_ = &param.type_.to_lib_type();
            quote!(
                $name: impl Into<#type_>
            )
        });

        let bindings_and_build = args.iter().map(|param| {
            let name = &param.name;
            let type_ = &param.type_.to_lib_type();
            quote!(
                let $name: #type_ = $name.into();
                __private_bindings.extend($name.get_bindings());
                __private_args.push($name.build());
            )
        });

        let bindings_and_build_clone = bindings_and_build.clone();

        let generated_function_def = quote!(
            pub fn #function_name(#( #function_params ), *) -> #crate_name::Function {
                use #crate_name::Buildable as _;
                use #crate_name::Parametric as _;
                {
                    let mut __private_bindings = vec![];
                    let mut __private_args = vec![];

                    #( #bindings_and_build )*

        // TODO: Confirm if a custom function has to be prefixed with 'fn::'
                let build = format!("{}({})", stringify!($function_name), __private_args.join(", "));
                #crate_name::Function::new()
                    .with_args_string(build)
                    .with_bindings(__private_bindings)
                }
            }

        );

        let mapped_field_types = args
            .iter()
            .map(|param| {
                let type_ = &param.type_.to_lib_type();
                quote!(#type_)
            })
            .collect::<Vec<_>>();
        let generated_func_macro = quote!(
                    // get_user!(56, 76, "username", "password");

                #[macro_use]
                macro_rules! #function_name {
                    ($($arg:expr),*) => {
                        {
                            use #crate_name::Buildable as _;
                            use #crate_name::Parametric as _;
                            {
                                let mut __private_bindings = vec![];
                                let mut __private_args = vec![];

                                #( #bindings_and_build_clone )*

                                let args = vec![$( $arg ),*];

                                for (index, arg) in args.iter().enumerate() {
                                    let type_ = &mapped_field_types[index];
                                    let $arg: $type_ = arg.into();
                                    __private_bindings.extend($arg.get_bindings());
                                    __private_args.push($arg.build());
                                }

                    // TODO: Confirm if a custom function has to be prefixed with 'fn::'
                            let build = format!("{}({})", stringify!($function_name), __private_args.join(", "));
                            #crate_name::Function::new()
                                .with_args_string(build)
                                .with_bindings(__private_bindings)
                            }

                        }
                    }
                 }
        );

        quote!(
            #define_function_statement
            #generated_function_def
            #generated_func_macro
        )
        .into()
    }
}

pub fn define_function(input: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(input as DefineFunctionStatementParser);
    function.tokenize()
}
