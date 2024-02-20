/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    visit::{self, Visit},
    Constraint, GenericArgument, Ident, Lifetime, Path, PathArguments, PathSegment, Type,
    TypeParamBound, WhereClause,
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
    pub edge_type: EdgeType,
}

// edge model rust type with generics
create_custom_type_wrapper!(EdgeType);

create_custom_type_wrapper!(EdgeTypeWithAggregatedGenerics);

impl EdgeType {
    pub fn aggregate_lifetime_and_generics_from_variations_of_a_type(
        edge_type_ident: &Ident,
        edge_types: &[Self],
    ) -> EdgeTypeWithAggregatedGenerics {
        let mut common_lifetimes = HashSet::new();
        let mut common_generics = HashSet::new();

        for edge_type in edge_types {
            let edge_type = &edge_type.to_basic_type();
            let mut visitor = UniqueTypeVisitor::default();
            visitor.visit_type(edge_type);

            for lt in visitor.lifetimes.iter() {
                common_lifetimes.insert(lt.to_string());
            }
            for gen in visitor.generics.iter() {
                common_generics.insert(gen.to_string());
            }
        }

        let lifetimes: Vec<Lifetime> = common_lifetimes
            .iter()
            .map(|lt| Lifetime::new(&format!("'{}", lt), proc_macro2::Span::call_site()))
            .collect();

        let generics: Vec<Ident> = common_generics
            .iter()
            .map(|gen| Ident::new(gen, proc_macro2::Span::call_site()))
            .collect();

        let quoted_type: TokenStream = if !lifetimes.is_empty() || !generics.is_empty() {
            let generics_list = quote! { <#(#lifetimes,)* #(#generics,)*> };
            quote! { #edge_type_ident #generics_list }
        } else {
            quote! { #edge_type_ident }
        };

        let edge_type = syn::parse2::<Type>(quoted_type).expect("Failed to parse edge type");

        edge_type.into()
    }
}

#[derive(Default)]
struct UniqueTypeVisitor {
    lifetimes: HashSet<String>,
    generics: HashSet<String>,
}

impl<'ast> Visit<'ast> for UniqueTypeVisitor {
    fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
        self.lifetimes.insert(i.ident.to_string());
        visit::visit_lifetime(self, i);
    }

    fn visit_type_param(&mut self, i: &'ast syn::TypeParam) {
        self.generics.insert(i.ident.to_string());
        visit::visit_type_param(self, i);
    }

    fn visit_type(&mut self, i: &'ast Type) {
        // Handle nested types by recursively visiting them
        visit::visit_type(self, i);
    }

    // Visit path arguments to handle generics in paths, e.g., `Vec<T>`
    fn visit_path_arguments(&mut self, path_arguments: &'ast PathArguments) {
        if let PathArguments::AngleBracketed(ref args) = path_arguments {
            for arg in &args.args {
                match arg {
                    GenericArgument::Type(Type::Path(type_path)) => {
                        self.visit_type_path(type_path);
                    }
                    GenericArgument::Lifetime(lt) => {
                        self.visit_lifetime(lt);
                    }
                    _ => {}
                }
            }
        }

        visit::visit_path_arguments(self, path_arguments);
    }

    // Visit paths, useful for extracting generics from qualified types
    fn visit_path(&mut self, i: &'ast Path) {
        for segment in &i.segments {
            self.visit_path_segment(segment);
        }
        visit::visit_path(self, i);
    }

    fn visit_path_segment(&mut self, segment: &'ast PathSegment) {
        if let PathArguments::AngleBracketed(ref args) = segment.arguments {
            for arg in &args.args {
                match arg {
                    GenericArgument::Type(ty) => {
                        self.visit_type(ty);
                    }
                    GenericArgument::Lifetime(lt) => {
                        self.visit_lifetime(lt);
                    }
                    // e.g., `T: Display` in `Foo<T: Display>`
                    GenericArgument::Constraint(constraint) => {
                        self.visit_constraint(constraint);
                    }
                    GenericArgument::Const(expr) => {}
                    _ => {}
                }
            }
        }

        visit::visit_path_segment(self, segment);
    }

    fn visit_constraint(&mut self, i: &'ast Constraint) {
        self.generics.insert(i.ident);
        for bound in i.bounds {
            self.visit_type_param_bound(&bound)
        }
    }

    // e.g: `T: Display` in generics
    fn visit_type_param_bound(&mut self, bound: &'ast TypeParamBound) {
        match &bound {
            TypeParamBound::Lifetime(lt) => {
                self.visit_lifetime(lt);
            }
            TypeParamBound::Trait(trait_bound) => {}
            TypeParamBound::Verbatim(_) => {}
        }

        visit::visit_type_param_bound(self, bound);
    }

    // Optionally handle `WhereClause` for capturing lifetimes and generics in trait bounds and where clauses
    fn visit_where_clause(&mut self, i: &'ast WhereClause) {
        visit::visit_where_clause(self, i);
    }
}

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
