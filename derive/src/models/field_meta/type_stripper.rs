/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use syn::{visit_mut::VisitMut, Lifetime, Type, TypeReference};

pub struct TypeStripper;

impl TypeStripper {
    pub fn strip_references_and_lifetimes(ty: &Type) -> Type {
        let mut stripped_type = ty.clone();
        TypeStripper.visit_type_mut(&mut stripped_type);
        stripped_type
    }
}

impl VisitMut for TypeStripper {
    fn visit_type_mut(&mut self, i: &mut Type) {
        if let Type::Reference(TypeReference { elem, .. }) = i {
            // Replace the type with the type it refers to
            *i = *elem.clone();
        }

        // Continue traversing the type
        syn::visit_mut::visit_type_mut(self, i);
    }
}
