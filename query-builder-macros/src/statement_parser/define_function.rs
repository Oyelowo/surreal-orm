use std::fmt::Display;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{all_consuming, cut},
    error::context,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{
    self,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Ident, Token,
};

use super::{helpers::generate_variable_name, if_else::Body};

#[derive(Clone, Debug)]
struct FieldTypeParser(FieldType);

impl Parse for FieldTypeParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_string = parse_until_top_level_comma(input)?;
        let type_ = type_string
            .parse::<FieldType>()
            .map_err(|_| syn::Error::new(Span::call_site(), "Invalid field type"))?;

        Ok(FieldTypeParser(type_))
    }
}

fn parse_until_top_level_comma(input: ParseStream) -> syn::Result<String> {
    let mut depth = 0;
    let mut type_string = String::new();

    while !input.is_empty() {
        if input.peek(Token![<]) {
            depth += 1;
            type_string.push('<');
            input.parse::<Token![<]>()?;
        } else if input.peek(Token![>]) && depth > 0 {
            depth -= 1;
            type_string.push('>');
            input.parse::<Token![>]>()?;
        } else if depth == 0 && input.peek(Token![,]) {
            break;
        } else {
            // Consume and append any other character
            let lookahead = input.fork();
            let ch: TokenTree = lookahead.parse()?;
            type_string.push_str(&ch.to_string());
            input.advance_to(&lookahead);
        }
    }

    Ok(type_string)
}

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

#[derive(Clone, Debug)]
struct Argument {
    name: Ident,
    type_: FieldTypeParser,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _arg_name: Ident = input.parse()?;
        let _ = input.parse::<Token![:]>()?; // Parsing the colon
        let type_ = input.parse::<FieldTypeParser>()?;

        Ok(Self {
            name: _arg_name,
            type_,
        })
    }
}

struct DefineFunctionStatementParser {
    function_name: Ident,
    args: Vec<Argument>,
    body: Body,
    generated_ident: Ident,
}

impl Parse for DefineFunctionStatementParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function_name = input.parse::<Ident>()?;

        let args_content;
        let _ = syn::parenthesized!(args_content in input);
        let parsed_args = args_content.parse_terminated(Argument::parse, Token![,])?;

        let body = input.parse::<Body>()?;

        Ok(Self {
            function_name,
            args: parsed_args.into_iter().collect::<Vec<_>>().into(),
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
                #crate_name::statements::FunctionArgument {
                     name: #name.into(),
                     type_: #type_.parse::<#crate_name::FieldType>().expect("Invalid field type.")
                }
            )
        }).collect::<Vec<_>>();

        let function_stmt_name = format_ident!("{function_name}_statement");

        let define_function_statement: proc_macro2::TokenStream = quote!(
            pub fn #function_stmt_name() -> #crate_name::statements::DefineFunctionStatement{
                #( #params_rendered )*

                #crate_name::statements::define_function(stringify!(#function_name))
                .arguments(::std::vec![#(#args_used),*])
                .body(#body)
            }
        );

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
            // #generated_function_def
            // #generated_func_macro
        )
        .into()
    }
}

pub fn define_function(input: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(input as DefineFunctionStatementParser);
    function.tokenize()
}
