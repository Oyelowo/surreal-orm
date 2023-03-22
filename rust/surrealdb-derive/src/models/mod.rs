/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub(crate) mod attributes;
pub(crate) mod casing;
pub(crate) mod edge;
pub(crate) mod errors;
pub(crate) mod node;
pub(crate) mod parser;
pub(crate) mod relations;
pub(crate) mod utils;
pub(crate) mod variables;
pub(crate) use utils::*;

use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

use syn::{self, Ident};
