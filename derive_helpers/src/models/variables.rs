/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use quote::format_ident;

pub struct VariablesModelMacro {
    /// This joins present model to the currently built graph.
    /// e.g Account->likes->Book.name
    /// For Node, this is usually just concatenating dot and the model fields i.e
    /// Mode.fieldname1, Model.fieldname2
    /// For edges, it usually surrounds the Edge with arrows e.g ->writes-> or <-writes<-
    /// Overall, this helps us do the graph traversal
    pub __________connect_node_to_graph_traversal_string: syn::Ident,
    pub __________connect_edge_to_graph_traversal_string: syn::Ident,
    pub __________connect_object_to_graph_traversal_string: syn::Ident,
    pub ___________bindings: syn::Ident,
    pub ___________errors: syn::Ident,
    pub ___________graph_traversal_string: syn::Ident,
    pub ____________update_many_bindings: syn::Ident,
    pub bindings: syn::Ident,
    pub schema_instance: syn::Ident,
    pub ___________model: syn::Ident,
    pub ___________in_marker: syn::Ident,
    pub ___________out_marker: syn::Ident,
    pub _____field_names: syn::Ident,
    pub _____struct_marker_ident: syn::Ident,
}

impl VariablesModelMacro {
    pub fn new() -> Self {
        let __________connect_node_to_graph_traversal_string =
            format_ident!("__________connect_node_to_graph_traversal_string");

        let __________connect_edge_to_graph_traversal_string =
            format_ident!("__________connect_edge_to_graph_traversal_string");

        let __________connect_object_to_graph_traversal_string =
            format_ident!("__________connect_object_to_graph_traversal_string");

        let ___________graph_traversal_string = format_ident!("___________graph_traversal_string");
        let ___________bindings = format_ident!("___________bindings");
        let ___________errors = format_ident!("___________errors");

        let bindings = format_ident!("bindings");
        let ____________update_many_bindings = format_ident!("____________update_many_bindings");

        let ___________model = format_ident!("___________model");
        let schema_instance = format_ident!("schema_instance");
        let ___________in_marker = format_ident!("___________in_marker");
        let ___________out_marker = format_ident!("___________out_marker");
        let _____field_names = format_ident!("_____field_names");

        Self {
            ___________graph_traversal_string,
            schema_instance,
            _____struct_marker_ident: format_ident!("_____struct_marker_ident"),
            ___________model,
            ___________in_marker,
            ___________out_marker,
            ___________bindings,
            _____field_names,
            ____________update_many_bindings,
            bindings,
            ___________errors,
            __________connect_node_to_graph_traversal_string,
            __________connect_edge_to_graph_traversal_string,
            __________connect_object_to_graph_traversal_string,
        }
    }
}
