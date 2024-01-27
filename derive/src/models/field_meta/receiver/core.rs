/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyeiayo
 * Licensed under the MIT license
 */

use surreal_query_builder::FieldType;
use syn::Ident;

use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, field_name_serialized::FieldNameSerialized,
        CaseString, CustomType, DataType, DbFieldType, DbFieldTypeAstMeta, RelationType,
    },
};

use super::{field_ident::NormalisedFieldMeta, MyFieldReceiver};

impl MyFieldReceiver {
    pub fn field_type_db(
        &self,
        table_attributes: &TableDeriveAttributes,
        model_type: &DataType,
    ) -> ExtractorResult<DbFieldType> {
        let db_type = match self.db_type {
            Some(ref db_type) => db_type.clone(),
            None => {
                let struct_level_casing = table_attributes.struct_casing();
                let field_name = &self
                    .normalize_ident(struct_level_casing)
                    .field_ident_serialized_fmt;
                let field_name = &self.field_name_serialized(table_attributes)?;
                let inferred = self
                    .ty
                    .infer_surreal_type_heuristically(
                        field_name,
                        &self.to_relation_type(),
                        model_type,
                    )
                    .map(|db_type_ast| db_type_ast.db_field_type)?;
                inferred
            }
        };
        Ok(db_type)
    }

    pub fn to_relation_type(&self) -> RelationType {
        self.into()
    }

    pub fn get_db_type(&self) -> ExtractorResult<DbFieldType> {
        // TODO: Handle error incase heuristics does not work and user does not specify
        Ok(self.db_type.clone())
    }

    pub fn get_db_type_with_assertion(
        &self,
        field_name: &FieldNameSerialized,
        model_type: &DataType,
        table: &Ident,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        // Infer/use user specified or error out
        // TODO: Add the compile time assertion/validations/checks for the dbtype here
        Ok(DbFieldTypeAstMeta {
            db_field_type: self.db_type,
            static_assertion: todo!(),
        })
    }

    pub fn is_numeric(&self) -> bool {
        let field_type = self.db_type.map_or(FieldType::Any, |t| t.into_inner());
        let explicit_db_ty_is_numeric = matches!(
            field_type,
            FieldType::Int | FieldType::Float | FieldType::Decimal | FieldType::Number
        );
        explicit_db_ty_is_numeric || self.rust_field_type().is_numeric()
    }

    pub fn is_list(&self) -> bool {
        let field_type = self.db_type.map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list =
            matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
        explicit_ty_is_list
            || self.rust_field_type().is_list()
            || self.db_type.map_or(false, |t| t.is_array())
            || self.link_many.is_some()
    }

    pub fn rust_field_type(&self) -> CustomType {
        self.ty
    }
}
