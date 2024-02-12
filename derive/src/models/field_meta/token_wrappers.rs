/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};

use crate::models::create_tokenstream_wrapper;

create_tokenstream_wrapper!(=> DbfieldTypeToken);

impl Default for DbfieldTypeToken {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self(quote!(#crate_name::FieldType::Any))
    }
}
create_tokenstream_wrapper!(=> SqlValueTokenStream);

create_tokenstream_wrapper!(=> StaticAssertionToken);

create_tokenstream_wrapper!(=> LinkedFields);
create_tokenstream_wrapper!(=> LinkOneFields);
create_tokenstream_wrapper!(=> LinkSelfFields);
create_tokenstream_wrapper!(=> LinkOneAndSelfFields);
create_tokenstream_wrapper!(=> LinkManyFields);
create_tokenstream_wrapper!(=> SerializableFields);

create_tokenstream_wrapper!(=> ConnectionWithFieldAppended);

create_tokenstream_wrapper!(=> AliasesStructFieldsTypesKv);
create_tokenstream_wrapper!(=> AliasesStructFieldsNamesKv);

create_tokenstream_wrapper!(=> FieldsRelationsAliased);

create_tokenstream_wrapper!(=> RenamedSerializedFields);
create_tokenstream_wrapper!(=> NonNullUpdaterFields);

create_tokenstream_wrapper!(=> TableIdType);

create_tokenstream_wrapper!(=> FieldMetadataToken);

create_tokenstream_wrapper!(=> SchemaStructFieldsTypesKv);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKv);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKvPrefixed);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKvEmpty);
