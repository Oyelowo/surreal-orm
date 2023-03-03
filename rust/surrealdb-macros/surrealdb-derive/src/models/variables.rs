use quote::format_ident;
use syn::Ident;

pub(crate) struct VariablesModelMacro {
    /// This joins present model to the currently built graph.
    /// e.g Account->likes->Book.name
    /// For SurrealdbNode, this is usually just concatenating dot and the model fields i.e
    /// Mode.fieldname1, Model.fieldname2
    /// For edges, it usually surrounds the SurrealdbEdge with arrows e.g ->writes-> or <-writes<-
    /// Overall, this helps us do the graph traversal
    pub __________connect_to_graph_traversal_string: syn::Ident,
    pub ___________bindings: syn::Ident,
    pub ___________graph_traversal_string: syn::Ident,
    pub schema_instance: syn::Ident,
    pub ___________model: syn::Ident,
    pub ___________in_marker: syn::Ident,
    pub ___________out_marker: syn::Ident,
    // Mainly used in edge schema to remove the start and end arrows for field access e.g
    // when we have "->writes->", we may want writes.time_written in case we want to access
    // a field on an edge itself because at the end of the day, an edge is a model or table
    // in the database itself
    pub schema_instance_edge_arrow_trimmed: syn::Ident,
}

impl VariablesModelMacro {
    pub fn new() -> Self {
        let __________connect_to_graph_traversal_string =
            format_ident!("__________connect_to_graph_traversal_string");
        let ___________graph_traversal_string = format_ident!("___________graph_traversal_string");
        let ___________bindings = format_ident!("___________bindings");

        let ___________model = format_ident!("___________model");
        let schema_instance = format_ident!("schema_instance");
        let ___________in_marker = format_ident!("___________in_marker");
        let ___________out_marker = format_ident!("___________out_marker");
        let schema_instance_edge_arrow_trimmed =
            format_ident!("schema_instance_edge_arrow_trimmed");

        Self {
            __________connect_to_graph_traversal_string,
            ___________graph_traversal_string,
            schema_instance,
            ___________model,
            schema_instance_edge_arrow_trimmed,
            ___________in_marker,
            ___________out_marker,
            ___________bindings,
        }
    }

    pub fn get_schema_alias(schema_name: &Ident) -> Ident {
        format_ident!("{schema_name}Schema")
    }
}
