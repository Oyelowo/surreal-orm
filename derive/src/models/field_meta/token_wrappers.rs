/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macros_helpers::get_crate_name;
use quote::quote;

use crate::models::*;

create_tokenstream_wrapper!(=> DbfieldTypeToken);

impl Default for DbfieldTypeToken {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self(quote!(#crate_name::FieldType::Any))
    }
}
create_tokenstream_wrapper!(=> SqlValueTokenStream);

create_tokenstream_wrapper!(=> StaticAssertionToken);

impl Default for StaticAssertionToken {
    fn default() -> Self {
        Self(quote!())
    }
}

create_tokenstream_wrapper!(=> LinkedField);
create_tokenstream_wrapper!(=> LinkOneField);
create_tokenstream_wrapper!(=> LinkSelfField);
create_tokenstream_wrapper!(=> LinkOneAndSelfField);
create_tokenstream_wrapper!(=> LinkManyField);
create_tokenstream_wrapper!(=> SerializableField);

create_tokenstream_wrapper!(=> ConnectionWithFieldAppended);

create_tokenstream_wrapper!(=> AliasesStructFieldsTypesKv);
create_tokenstream_wrapper!(=> AliasesStructFieldsNamesKv);

create_tokenstream_wrapper!(=> FieldsRelationsAliased);

create_tokenstream_wrapper!(=> RenamedSerializedFields);
create_tokenstream_wrapper!(=> NonNullUpdaterFields);

create_tokenstream_wrapper!(=> TableIdType);
impl Default for TableIdType {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self(quote!())
    }
}

create_tokenstream_wrapper!(=> FieldMetadataToken);

create_tokenstream_wrapper!(=> SchemaStructFieldsTypesKv);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKv);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKvPrefixed);
create_tokenstream_wrapper!(=> SchemaStructFieldsNamesKvEmpty);

create_tokenstream_wrapper!(=> DbFieldNamesToken);

create_tokenstream_wrapper!(=> TableDefinitions);

create_tokenstream_wrapper!(=> DefineFieldStatementToken);
