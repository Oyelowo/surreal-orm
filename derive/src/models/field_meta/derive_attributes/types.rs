/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;
use surreal_query_builder::FieldType;
use syn::Ident;

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn field_type_db(
        &self,
        model_attributes: &impl ModelAttributes,
        model_type: &DataType,
    ) -> ExtractorResult<FieldTypeDb> {
        let db_type = match self.field_type_db {
            Some(ref db_type) => db_type.clone(),
            None => {
                let casing = model_attributes.casing()?;
                let field_name = &self.db_field_name(&casing)?;
                let inferred = self
                    .ty()
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
            field_type_db: self.field_type_db.expect("Not yet implemented!"),
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
        explicit_db_ty_is_numeric || self.ty().is_numeric()
    }

    pub fn is_array(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list = matches!(field_type, FieldType::Array(item_ty, _));
        explicit_ty_is_list
            || self.ty().is_array()
            || self.field_type_db.map_or(false, |t| t.is_array())
            || self.link_many.is_some()
    }

    pub fn is_set(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list = matches!(field_type, FieldType::Set(item_ty, _));
        explicit_ty_is_list
            || self.ty().is_set()
            || self.field_type_db.map_or(false, |t| t.is_set())
    }

    pub fn is_list(&self) -> bool {
        let field_type = self
            .field_type_db
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list =
            matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
        explicit_ty_is_list
            || self.ty().is_list()
            || self.field_type_db.map_or(false, |t| t.is_list())
            || self.link_many.is_some()
    }

    pub fn ty(&self) -> CustomType {
        self.ty.into()
    }
}
