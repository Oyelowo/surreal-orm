/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use syn::Type;

use crate::models::{
    create_custom_type_wrapper, CustomType, DestinationNodeTypeOriginal, RustFieldTypeSelfAllowed,
};

#[derive(Debug, Clone)]
pub struct Relate {
    /// e.g ->writes->book
    pub connection: String,
    // #[darling(default)]
    /// e.g StudentWritesBook,
    /// derived from: type StudentWritesBook = Writes<Student, Book>;
    /// e.g2
    /// StudentWritesBook<'a, 'b: 'a, T, U>,
    /// derived from: type StudentWritesBook<'a, 'b: 'a, T, U> = Writes<'a, 'b: 'a, T, U><Student<'a, 'b, T, Book<U>>;
    pub edge_type: EdgeModel,
}

create_custom_type_wrapper!(EdgeModel);

impl FromMeta for Relate {
    // // TODO: Revisit this whether we can and should allow only the
    // model to be specified if we can infer the connection direction at
    // compile time.
    // fn from_string(value: &str) -> darling::Result<Self> {
    //     Ok(Self {
    //         connection: value.into(),
    //         model: None,
    //     })
    // }

    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        // Todo: Just use Rrelate alone if we dont have to specify connection direction
        // explicitly
        #[derive(FromMeta)]
        struct FullRelate {
            model: Type,
            connection: String,
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate {
                    connection, model, ..
                } = v;
                Self {
                    connection,
                    edge_type: model,
                }
            }
        }
        FullRelate::from_list(items).map(Relate::from)
    }
}
