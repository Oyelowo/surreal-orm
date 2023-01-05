use quote::__private::Span;
use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;

#[derive(Debug, Clone)]
pub enum Field {
  Property(FieldProperty),
  ForeignNode(FieldForeignNode),
  Relation(FieldRelation),
}

impl Field {
  pub fn emit_field(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_field(),
      Field::ForeignNode(x) => x.emit_field(),
      Field::Relation(x) => x.emit_field(),
    }
  }

  pub fn emit_initialization(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization(),
      Field::ForeignNode(x) => x.emit_initialization(),
      Field::Relation(x) => x.emit_initialization(),
    }
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization_with_origin(),
      Field::ForeignNode(x) => x.emit_initialization_with_origin(),
      Field::Relation(x) => x.emit_initialization_with_origin(),
    }
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_foreign_field_function(),
      Field::ForeignNode(x) => x.emit_foreign_field_function(),
      Field::Relation(x) => x.emit_foreign_field_function(),
    }
  }
}

/// A simple property
#[derive(Debug, Clone)]
pub struct FieldProperty {
  pub name: String,

  pub is_public: bool,
}

// #[derive(Debug, Clone, Copy)]
// enum Keyword {
//   In,
// }
//
// impl From<Keyword> for &str {
//   fn from(value: Keyword) -> Self {
//     match value {
//       Keyword::In => "in",
//     }
//   }
// }
//
// impl From<&String> for FieldName {
//   fn from(value: &String) -> Self {
//     match value.as_str() {
//       "in" | "r#in" => FieldName::Keyword(Keyword::In),
//       v => FieldName::Others(v.to_string()),
//     }
//   }
// }
// #[derive(Debug, Clone)]
// enum FieldName {
//   Keyword(Keyword),
//   Others(String),
// }
fn get_ident(name: &String) -> syn::Ident {
  // let xx = match FieldName::from(name) {
  //   FieldName::Keyword(x) => syn::Ident::new_raw(x.into(), Span::call_site()),
  //   FieldName::Others(o) => syn::Ident::new(o.as_str(), Span::call_site()),
  // };
  // if name == &"in".to_string() {

  if vec!["in", "r#in"].contains(&name.as_str()) {
    syn::Ident::new_raw(name.trim_start_matches("r#"), Span::call_site())
  // } else if name == "r#in" {
  // syn::Ident::new("in", Span::call_site())
  } else {
    syn::Ident::new(name.as_str(), Span::call_site())
  }
}

impl FieldProperty {
  fn emit_field(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #name: SchemaField<N>
    )
    .into()
  }

  pub fn emit_initialization(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    quote!()
  }
}

/// A foreign node, like a foreign key that points to another `Model`
#[derive(Debug, Clone)]
pub struct FieldForeignNode {
  pub name: String,
  pub foreign_type: String,

  pub is_public: bool,
}

impl FieldForeignNode {
  fn emit_field(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #name: SchemaField<N>
    )
  }

  pub fn emit_initialization(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    // let name = format_ident!("{}", self.name);
    let name = get_ident(&self.name);
    let foreign_type = format_ident!("{}", self.foreign_type);

    quote!(
      pub fn #name (self) -> #foreign_type <{ N + 2 }> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_origin: [&'static str; N + 2] = [""; N + 2];
        new_origin[..N].clone_from_slice(&origin.segments);

        if (N > 0 && new_origin[N - 1] != ".") {
          new_origin[N] = ".";
        }

        new_origin[N + 1] = self.#name.identifier;

        #foreign_type::with_origin(OriginHolder::new(new_origin))
      }
    )
  }
}

/// A named relation
#[derive(Debug, Clone)]
pub struct FieldRelation {
  pub name: String,
  pub foreign_type: String,
  pub alias: String,
  pub relation_type: FieldRelationType,
  pub is_public: bool,
}

#[derive(Debug, Clone)]
pub enum FieldRelationType {
  /// for `->` type of relations/edges
  OutgoingEdge,

  /// for `<-` type of relations/edges
  IncomingEdge,
}

impl FieldRelation {
  fn emit_field(&self) -> TokenStream {
    let alias = format_ident!("{}", self.alias);
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #alias: SchemaField<N>
    )
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let alias = format_ident!("{}", self.alias);
    let name_str = format!("{}{}{}", self.name, self.edge(), self.foreign_type);
    let field_type = self.field_type();

    quote!(#alias: SchemaField::new(#name_str, #field_type))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let alias = format_ident!("{}", self.alias);
    let name_str = format!("{}{}{}", self.name, self.edge(), self.foreign_type);
    let field_type = self.field_type();

    quote!(#alias: SchemaField::with_origin(#name_str, #field_type, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let alias = format_ident!("{}", self.alias);
    let foreign_type = format_ident!("{}", self.foreign_type);
    let edge = self.edge();

    quote!(
      pub fn #alias (self) -> #foreign_type <{N + 2}> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_nested_origin: [&'static str; N + 2] = [""; N + 2];
        new_nested_origin[..N].clone_from_slice(&origin.segments);

        new_nested_origin[N] = #edge;
        new_nested_origin[N + 1] = self.#alias.identifier;

        #foreign_type::with_origin(OriginHolder::new(new_nested_origin))
      }
    )
  }

  fn edge(&self) -> &'static str {
    match &self.relation_type {
      FieldRelationType::OutgoingEdge => "->",
      FieldRelationType::IncomingEdge => "<-",
    }
  }

  fn field_type(&self) -> TokenStream {
    match &self.relation_type {
      FieldRelationType::OutgoingEdge => quote!(SchemaFieldType::Relation),
      FieldRelationType::IncomingEdge => quote!(SchemaFieldType::ForeignRelation),
    }
  }
}

fn emit_skip_serializing_attribute() -> TokenStream {
  quote!(#[serde(skip_serializing)])
}
