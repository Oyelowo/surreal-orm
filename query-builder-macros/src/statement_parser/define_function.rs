use std::fmt::Display;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::{parse_field_type, parse_top_level_field_type, FieldType};
use syn::{
    self,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Ident, Token,
};

use super::{helpers::generate_variable_name, if_else::Body};

#[derive(Debug)]
struct FieldTypeParser(FieldType);

impl From<FieldType> for FieldTypeParser {
    fn from(value: FieldType) -> Self {
        Self(value)
    }
}

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

// (first: string, sec: option<array<int, 5>>, third: array<string, 5>)
// impl Parse for FieldTypeParser {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         input.parse::<Ident>();
//         let type_ = input.to_string().trim().parse::<FieldType>();
//         match type_ {
//             Ok(type_) => Ok(Self(type_)),
//             Err(_) => Err(syn::Error::new(
//                 Span::call_site(),
//                 "expected a valid field type",
//             )),
//         }
//     }
// }

// A parser for an identifier.
fn parse_identifier(input: &str) -> IResult<&str, &str> {
    let (input, ident) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    Ok((input, ident))
}

#[derive(Debug)]
struct Argument2 {
    name: Ident,
    type_: FieldTypeParser,
}
// fn parse_function_argument(input: &str) -> IResult<&str, (&str, FieldType)> {
fn parse_function_argument(i: &str) -> IResult<&str, Argument2> {
    let (i, _) = multispace0(i)?;
    let (i, name) = parse_identifier(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace0(i)?;
    // let (i, type_) = parse_field_type(i)?;
    let (i, type_) = parse_top_level_field_type(i)?;
    let (i, _) = multispace0(i)?;

    // let (input, (_, name, _, _, _, type_, _)) = tuple((
    //     multispace0,      // Handles optional spaces.
    //     parse_identifier, // Parses the argument name.
    //     multispace0,      // Handles optional spaces.
    //     tag(":"),         // Starting delimiter for the type.
    //     multispace0,      // Handles optional spaces.
    //     parse_field_type, // Parses the FieldType.
    //     multispace0,      // Handles optional trailing spaces.
    // ))(i)?;
    // let (input, (name, _, type_)) = tuple((
    //     parse_identifier, // Parses the argument name.
    //     multispace0,      // Handles optional spaces.
    //     delimited(
    //         tag(":"),         // Starting delimiter for the type.
    //         parse_field_type, // Parses the FieldType.
    //         multispace0,      // Handles optional trailing spaces.
    //     ),
    // ))(input)?;

    // Ok((input, (name, type_str)))
    Ok((
        i,
        Argument2 {
            name: format_ident!("{name}"),
            type_: type_.into(),
        },
    ))
}

fn parse_function_arguments(input: &str) -> IResult<&str, Vec<Argument2>> {
    let (input, _) = multispace0(input)?;
    let res = separated_list0(tag(","), parse_function_argument)(input);
    let (input, _) = multispace0(input)?;

    Ok((input, res.unwrap().1))
}

// Example usage within a function signature parser.
// fn parse_function_signature(input: &str) -> IResult<&str, Vec<(&str, FieldType)>> {
//     delimited(char('('), parse_function_arguments, char(')'))(input)
// }

impl Parse for FieldTypeParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // // let _arg_name: Ident = input.parse()?;
        // // input.parse::<Token![:]>()?; // Parsing the colon
        //
        // let mut type_str = String::new();
        // let mut successful_parse = false;
        //
        // while !input.is_empty() {
        //     let lookahead = input.fork();
        //     let chunk: proc_macro2::TokenTree = lookahead.parse()?;
        //     // panic!("chunkala. debug-{:?}...print-{}", chunk, chunk.to_string());
        //     type_str.push_str(&chunk.to_string());
        //
        //     if let Ok(ft) = type_str.parse::<FieldType>() {
        //         successful_parse = true;
        //         input.advance_to(&lookahead);
        //         break;
        //     }
        //
        //     // Consume the chunk including the comma if present
        //     if input.peek(Token![,]) {
        //         type_str.push(',');
        //         input.parse::<Token![,]>()?;
        //     } else {
        //         input.advance_to(&lookahead);
        //     }
        // }
        //
        // panic!("type_str: {}", type_str);
        // if successful_parse {
        //     Ok(FieldTypeParser(
        //         type_str.parse::<FieldType>().expect("problem"),
        //     ))
        // } else {
        //     Err(syn::Error::new(
        //         Span::call_site(),
        //         "Unable to parse field type",
        //     ))
        // }

        todo!()
    }
}

struct Argument {
    name: Ident,
    separator: Token![:],
    type_: FieldTypeParser,
}

impl Parse for Argument {
    // e.g first_arg: string
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let sep = input.parse::<Token![:]>()?;
        let type_ = input.parse::<FieldTypeParser>()?;
        Ok(Self {
            name,
            separator: sep,
            type_,
        })
    }
}

struct DefineFunctionStatementParser {
    function_name: Ident,
    args: Vec<Argument2>,
    body: Body,
    generated_ident: Ident,
}

impl Parse for DefineFunctionStatementParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function_name = input.parse::<Ident>()?;

        let args_content;
        let _ = syn::parenthesized!(args_content in input);
        let args_content = args_content.to_string();
        // panic!("args_content: {}", args_content);
        let parsed_args =
            // parse_function_arguments("first_arg: option<array<int, 5>> | int | set<string, 53>")
            // parse_function_arguments("first_arg: string, second: int, third: set<string, 53>, fourth: option<array<int, 5>>")
            parse_function_arguments(&args_content)
                .expect("Invalid argument...")
                .1;

        panic!(
            "parsed_args: {:?}",
            parsed_args
                .into_iter()
                .map(|arg| arg.type_)
                .collect::<Vec<_>>()
        );
        let body = input.parse::<Body>()?;

        Ok(Self {
            function_name,
            // args: params.into_iter().collect(),
            args: parsed_args,
            // args: xx,
            body,
            generated_ident: generate_variable_name(),
        })
    }
}

impl DefineFunctionStatementParser {
    pub fn tokenize(&self) -> TokenStream {
        let Self {
            function_name,
            args,
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
