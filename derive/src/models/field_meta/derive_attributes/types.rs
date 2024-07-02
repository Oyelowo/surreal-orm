/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;
use surreal_query_builder::FieldType;

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn field_type_db(
        &self,
        model_attributes: &ModelAttributes,
    ) -> ExtractorResult<FieldTypeDb> {
        self.field_type_db_with_static_assertions(model_attributes)
            .map(|x| x.field_type_db_original.into())
    }

    pub fn field_type_db_with_static_assertions(
        &self,
        model_attributes: &ModelAttributes,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let db_type = match self.field_type_db {
            Some(ref db_type) => DbFieldTypeAstMeta {
                field_type_db_original: db_type.clone().into_inner(),
                field_type_db_token: db_type.clone(),
                static_assertion_token: db_type.generate_static_assertion(&self.ty()).into(),
            },
            None => {
                let casing = model_attributes.casing()?;
                let field_name = &self.db_field_name(&casing)?;

                let inferred = FieldTypeInference {
                    db_field_name: field_name,
                    relation_type: &self.to_relation_type(),
                    field_ty: &self.ty(),
                    model_attrs: model_attributes,
                }
                .infer_type()
                .map(|ft_db| ft_db)?;

                inferred
            }
        };

        Ok(db_type)
    }

    pub fn to_relation_type(&self) -> RelationType {
        self.into()
    }

    pub fn is_numeric(&self) -> bool {
        let field_type = self
            .field_type_db
            .as_ref()
            .map_or(&FieldType::Any, |t| t.into_inner_ref());
        let explicit_db_ty_is_numeric = matches!(
            field_type,
            FieldType::Int | FieldType::Float | FieldType::Decimal | FieldType::Number
        );
        explicit_db_ty_is_numeric || self.ty().is_numeric()
    }

    pub fn is_array(&self) -> bool {
        let field_type = self
            .field_type_db
            .as_ref()
            .map_or(&FieldType::Any, |t| t.into_inner_ref());
        let explicit_ty_is_list = matches!(field_type, FieldType::Array(_item_ty, _));
        explicit_ty_is_list
            || self.ty().is_array()
            || self.field_type_db.as_ref().map_or(false, |t| t.is_array())
            || self.link_many.is_some()
    }

    pub fn is_set(&self) -> bool {
        let field_type = self
            .field_type_db
            .as_ref()
            .map_or(&FieldType::Any, |t| t.into_inner_ref());
        let explicit_ty_is_list = matches!(field_type, FieldType::Set(_item_ty, _));
        explicit_ty_is_list
            || self.ty().is_set()
            || self.field_type_db.as_ref().map_or(false, |t| t.is_set())
    }

    // TODO: Remove this?
    // pub fn is_list(&self) -> bool {
    //     let field_type = self
    //         .field_type_db
    //         .as_ref()
    //         .map_or(&FieldType::Any, |t| t.into_inner_ref());
    //     let explicit_ty_is_list =
    //         matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
    //     explicit_ty_is_list
    //         || self.ty().is_list()
    //         || self.field_type_db.as_ref().map_or(false, |t| t.is_list())
    //         || self.link_many.is_some()
    // }

    pub fn ty(&self) -> CustomType {
        CustomType::new(self.ty.clone())
    }
}
