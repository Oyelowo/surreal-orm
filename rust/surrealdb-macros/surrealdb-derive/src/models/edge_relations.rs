use super::edge::{MyFieldReceiver, Relate};

#[derive(Debug, Clone)]
pub(crate) enum RelationType {
    LinkOne(NodeName),
    LinkSelf(NodeName),
    LinkMany(NodeName),
    None,
}

impl From<&MyFieldReceiver> for RelationType {
    fn from(field_receiver: &MyFieldReceiver) -> Self {
        match field_receiver {
            MyFieldReceiver {
                link_one: Some(link_one),
                ..
            } => RelationType::LinkOne(link_one.into()),
            MyFieldReceiver {
                link_self: Some(link_self),
                ..
            } => RelationType::LinkSelf(link_self.into()),
            MyFieldReceiver {
                link_many: Some(link_many),
                ..
            } => RelationType::LinkMany(link_many.into()),
            _ => RelationType::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EdgeDirection {
    OutArrowRight,
    InArrowLeft,
}

impl From<EdgeDirection> for ::proc_macro2::TokenStream {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => quote::quote!(->),
            EdgeDirection::InArrowLeft => quote::quote!(<-),
        }
    }
}

impl From<EdgeDirection> for String {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => "->".into(),
            EdgeDirection::InArrowLeft => "<-".into(),
        }
    }
}

macro_rules! wrapper_struct_to_ident {
    ($simple_wrapper_struct:ty) => {
        impl From<$simple_wrapper_struct> for ::proc_macro2::TokenStream {
            fn from(simple_wrapper_struct: $simple_wrapper_struct) -> Self {
                let ident = ::quote::format_ident!("{}", simple_wrapper_struct.0);
                ::quote::quote!(#ident)
            }
        }

        impl ::std::fmt::Display for $simple_wrapper_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.0))
            }
        }
    };
}

/*
impl From<EdgeAction> for TokenStream {
    fn from(edge_action: EdgeAction) -> Self {
        let action = format_ident!("{}", edge_action.0);
        quote!(#action)
    }
}
*/

#[derive(Debug, Clone)]
pub(crate) struct EdgeName(String);
wrapper_struct_to_ident!(EdgeName);

#[derive(Debug, Clone)]
pub(crate) struct NodeName(String);

impl From<&String> for NodeName {
    fn from(value: &String) -> Self {
        Self(value.into())
    }
}
impl From<NodeName> for String {
    fn from(value: NodeName) -> Self {
        value.0
    }
}
wrapper_struct_to_ident!(NodeName);

#[derive(Debug, Clone)]
pub(crate) struct RelateAttribute {
    pub(crate) edge_direction: EdgeDirection,
    pub(crate) edge_name: EdgeName,
    pub(crate) node_name: NodeName,
}

impl From<RelateAttribute> for ::proc_macro2::TokenStream {
    fn from(relate_attrs: RelateAttribute) -> Self {
        let edge_direction = ::proc_macro2::TokenStream::from(relate_attrs.edge_direction);
        let edge_name = ::proc_macro2::TokenStream::from(relate_attrs.edge_name);
        let node_name = ::proc_macro2::TokenStream::from(relate_attrs.node_name);
        // ->action->NodeObject
        // <-action<-NodeObject
        // e.g ->manages->Project
        ::quote::quote!(#edge_direction #edge_name #node_name)
    }
}

// #[derive(Debug, Clone)]
// pub(crate) struct Relation(pub Relate);

// impl From<Relation> for String {
//     fn from(relation: Relation) -> Self {
//         relation.0.link
//     }
// }
// impl From<String> for Relation {
//     fn from(str: String) -> Self {
//         Relation(Relate { link: str })
//     }
// }

// impl From<Relation> for RelateAttribute {
impl From<Relate> for RelateAttribute {
    fn from(relation: Relate) -> Self {
        let right_arrow_count = relation.link.matches("->").count();
        let left_arrow_count = relation.link.matches("<-").count();
        let edge_direction = match (left_arrow_count, right_arrow_count) {
            (2, 0) => EdgeDirection::InArrowLeft,
            (0, 2) => EdgeDirection::OutArrowRight,
            _ => panic!("Arrow incorrectly used"),
        };

        let edge_direction_str: String = edge_direction.into();
        let mut substrings = relation
            .link
            .split(edge_direction_str.as_str())
            .filter(|x| !x.is_empty());

        let (edge_action, node_object) =
            match (substrings.next(), substrings.next(), substrings.next()) {
                (Some(action), Some(node_obj), None) => {
                    (EdgeName(action.into()), NodeName(node_obj.into()))
                }
                _ => panic!(
                    "too many actions or object, {}",
                    get_relation_error(&relation)
                ),
            };

        Self {
            node_name: node_object,
            edge_name: edge_action,
            edge_direction,
        }
    }
}

fn get_relation_error<'a>(_relation: &Relate) -> ::std::fmt::Arguments<'a> {
    // let span = syn::spanned::Spanned::span(relation.0.clone()).clone();
    // let span = syn::spanned::Spanned::span(relation.0.as_str()).clone();

    // let start = span.clone().start().clone();
    // let end = span.clone().end().clone();
    // let start_line = start.line;
    // let start_column = start.column;
    // let end_column = end.column;
    let c = format_args!(
        " Check that your arrows are properly faced. e.g ->has->Heart or <-owned_by<-Human",
        // start_line,
        // start_column,
        // end_column
    );
    c
}
