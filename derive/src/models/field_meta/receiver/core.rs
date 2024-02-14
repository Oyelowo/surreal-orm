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
        derive_attributes::TableDeriveAttributes, field_name_serialized::DbFieldName, CaseString,
        CustomType, DataType, DbFieldTypeAstMeta, FieldTypeDb, RelationType,
    },
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn field_type_db(
        &self,
        table_attributes: &TableDeriveAttributes,
        model_type: &DataType,
    ) -> ExtractorResult<FieldTypeDb> {
        let db_type = match self.field_type_db {
            Some(ref db_type) => db_type.clone(),
            None => {
                let casing = table_attributes.casing()?;
                let field_name = &self.field_name_serialized(&casing)?;
                let inferred = self
                    .ty
                    .infer_surreal_type_heuristically(
                        field_name,
                        &self.to_relation_type(),
                        model_type,
                    )
                    .map(|ft_db| ft_db.field_type_db)?;
                inferred
            }
        };
        Ok(db_type)
    }

    pub fn to_relation_type(&self) -> RelationType {
        self.into()
    }

    pub fn field_type_and_assertion(
        &self,
        field_name: &DbFieldName,
        model_type: &DataType,
        table: &Ident,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        // Infer/use user specified or error out
        // TODO: Add the compile time assertion/validations/checks for the dbtype here
        Ok(DbFieldTypeAstMeta {
            field_type_db: self.field_type_db,
            static_assertion: todo!(),
        })
    }

    pub fn is_numeric(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_db_ty_is_numeric = matches!(
            field_type,
            FieldType::Int | FieldType::Float | FieldType::Decimal | FieldType::Number
        );
        explicit_db_ty_is_numeric || self.field_type_rust().is_numeric()
    }

    pub fn is_array(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list = matches!(field_type, FieldType::Array(item_ty, _));
        explicit_ty_is_list
            || self.field_type_rust().is_array()
            || self.field_type_db.map_or(false, |t| t.is_array())
            || self.link_many.is_some()
    }

    pub fn is_set(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list = matches!(field_type, FieldType::Set(item_ty, _));
        explicit_ty_is_list
            || self.field_type_rust().is_set()
            || self.field_type_db.map_or(false, |t| t.is_set())
    }

    pub fn is_list(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list = matches!(
            field_type,
            FieldType::Array(item_ty, _) | FieldType::Set(_, _)
        );
        explicit_ty_is_list
            || self.field_type_rust().is_list()
            || self.field_type_db.map_or(false, |t| t.is_list())
            || self.link_many.is_some()
    }

    pub fn field_type_rust(&self) -> CustomType {
        self.ty
    }
}
