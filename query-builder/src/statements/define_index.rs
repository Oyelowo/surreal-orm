/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Statement syntax
// DEFINE INDEX statement
// Just like in other databases, SurrealDB uses indexes to help optimize query performance. An index can consist of one or more fields in a table and can enforce a uniqueness constraint. If you don't intend for your index to have a uniqueness constraint, then the fields you select for your index should have a high degree of cardinality, meaning that there is a high amount of diversity between the data in the indexed table records.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE INDEX statement.
// You must select your namespace and database before you can use the DEFINE INDEX statement.
// Statement syntax
// DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields
// 	[ UNIQUE | SEARCH ANALYZER @analyzer [ BM25 [(@k1, @b)] ] [ HIGHLIGHTS ] ]
// Example usage
// How to create a unique index for the email address field on a user table.
//
// -- Make sure that email addresses in the user table are always unique
// DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
// How to create a non-unique index for an age field on a user table.
//
// -- optimise queries looking for users of a given age
// DEFINE INDEX userAgeIndex ON TABLE user COLUMNS age;
// How to create a full-text search index for a name field on a user table.
//
// -- Allow full-text search queries on the name of the user
// DEFINE INDEX userNameIndex ON TABLE user COLUMNS name SEARCH ANALYZER ascii BM25 HIGHLIGHTS;

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Field, TableIndex},
    ErrorList, NumberLike, TableLike, ValueLike,
};

/// Define a new database index.
/// Just like in other databases, SurrealDB uses indexes to help optimize query performance.
/// An index can consist of one or more fields in a table and can enforce a uniqueness constraint.
/// If you don't intend for your index to have a uniqueness constraint,
/// then the fields you select for your index should have a high degree of cardinality,
/// meaning that there is a high amount of diversity between the data in the indexed table records.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE INDEX statement.
/// You must select your namespace and database before you can use the DEFINE INDEX statement.
///
/// Example:
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_index}};
/// # let alien = Table::from("alien");
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let email = Field::new("email");
/// # let dob = Field::new("dob");
///
/// let query = define_index("alien_index")
///                 .on_table(alien)
///                 .fields(&[age, name, email, dob])
///                 .unique();
///
/// assert_eq!(query.to_raw().build(),
/// "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;");
/// ```
pub fn define_index(index_name: impl Into<TableIndex>) -> DefineIndexStatement {
    let index_name: TableIndex = index_name.into();
    let index_name: String = index_name.to_string();

    DefineIndexStatement {
        index_name,
        table: None,
        fields: vec![],
        columns: vec![],
        unique: None,
        search_analyzer: None,
        bindings: vec![],
        errors: vec![],
    }
}

pub enum Columns {
    Field(Field),
    Fields(Vec<Field>),
}

pub type Fields = Columns;

impl From<Field> for Columns {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Columns {
    fn from(value: &[Field; N]) -> Self {
        Self::Fields(value.iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<[Field; N]> for Columns {
    fn from(value: [Field; N]) -> Self {
        Self::Fields(value.to_vec())
    }
}

impl<const N: usize> From<[ValueLike; N]> for Columns {
    fn from(value: [ValueLike; N]) -> Self {
        Self::Fields(
            value
                .into_iter()
                .map(|v| {
                    Field::new(v.build())
                        .with_bindings(v.get_bindings())
                        .with_errors(v.get_errors())
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Vec<ValueLike>> for Columns {
    fn from(value: Vec<ValueLike>) -> Self {
        Self::Fields(
            value
                .into_iter()
                .map(|v| {
                    Field::new(v.build())
                        .with_bindings(v.get_bindings())
                        .with_errors(v.get_errors())
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Vec<Field>> for Columns {
    fn from(value: Vec<Field>) -> Self {
        Self::Fields(value)
    }
}

impl Parametric for Columns {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Columns::Field(field) => field.get_bindings(),
            Columns::Fields(fields) => fields
                .iter()
                .flat_map(|f| f.get_bindings())
                .collect::<Vec<_>>(),
        }
    }
}

/// A statement for defining a database Index.
pub struct DefineIndexStatement {
    index_name: String,
    table: Option<String>,
    fields: Vec<Field>,
    columns: Vec<Field>,
    unique: Option<bool>,
    search_analyzer: Option<SearchAnalyzer>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl DefineIndexStatement {
    /// Set the table where the index is defined.
    pub fn on_table(mut self, table: impl Into<TableLike>) -> Self {
        let table: TableLike = table.into();
        self.bindings.extend(table.get_bindings());
        self.errors.extend(table.get_errors());
        self.table = Some(table.build());
        self
    }

    /// Set the columns on the table where the index should be defined. This is alternative to
    /// fields just like in a relational database
    pub fn columns(mut self, columns: impl Into<Columns>) -> Self {
        let columns: Columns = columns.into();
        let columns = match columns {
            Columns::Field(f) => vec![f],
            Columns::Fields(fs) => fs,
        };
        self.columns.extend(columns);
        // self.bindings.extend(columns.get_bindings());
        // self.errors.extend(columns.get_errors());
        self
    }

    /// Set the fields on the table where the index should be defined. This is alternative to
    /// columns
    pub fn fields(mut self, fields: impl Into<Fields>) -> Self {
        let fields: Fields = fields.into();
        let fields = match fields {
            Fields::Field(f) => vec![f],
            Fields::Fields(fs) => fs,
        };
        self.fields.extend(fields);
        // self.bindings.extend(fields.get_bindings());
        // self.errors.extend(fields.get_errors());
        self
    }

    /// Set whether the field should be unique
    pub fn unique(mut self) -> Self {
        self.unique = Some(true);
        self
    }

    /// Set the search analyzer for the index
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{statements::search_analyzer};
    ///
    /// search_analyzer("my_analyzer")
    ///   .highlight()
    ///   .bm25(1.2, 0.75)
    ///   .doc_ids_order(1)
    ///   .doc_lengths_order(1)
    ///   .postings_order(1)
    ///   .terms_order(1);
    pub fn search_analyzer(mut self, search_analyzer: SearchAnalyzer) -> Self {
        self.search_analyzer = Some(search_analyzer);
        // self.bindings.extend(search_analyzer.get_bindings());
        // self.errors.extend(search_analyzer.get_errors());
        self
    }
}

impl Buildable for DefineIndexStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE INDEX {}", self.index_name);

        if let Some(table) = &self.table {
            query = format!("{query} ON TABLE {table}");
        }

        if !self.fields.is_empty() {
            let fields_str = self
                .fields
                .iter()
                .map(|f| f.build())
                .collect::<Vec<_>>()
                .join(", ");
            query = format!("{query} FIELDS {fields_str}");
        }

        if !self.columns.is_empty() {
            let columns_str = self
                .columns
                .iter()
                .map(|f| f.build())
                .collect::<Vec<_>>()
                .join(", ");
            query = format!("{query} COLUMNS {columns_str}");
        }

        if self.unique.unwrap_or(false) {
            query = format!("{query} UNIQUE");
        } else if let Some(search_analyzer) = &self.search_analyzer {
            let search_analyzer = search_analyzer.build();
            query = format!("{query} {search_analyzer}");
        }
        query += ";";
        query
    }
}

impl Display for DefineIndexStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineIndexStatement {
    fn get_bindings(&self) -> BindingsList {
        let mut bindings = self.bindings.clone();

        if !self.fields.is_empty() {
            bindings.extend(self.fields.get_bindings());
        }

        if !self.columns.is_empty() {
            bindings.extend(self.columns.get_bindings());
        }
        if let Some(search_analyzer) = &self.search_analyzer {
            bindings.extend(search_analyzer.get_bindings());
        }

        bindings
    }
}

impl Queryable for DefineIndexStatement {}
impl Erroneous for DefineIndexStatement {}

/// Scoring for search
pub enum Scoring {
    // Bm { k1: NumberLike, b: NumberLike }, // BestMatching25
    /// BestMatching25
    Bm25(NumberLike, NumberLike),
    /// VectorSearch
    Vs,
}

impl Buildable for Scoring {
    fn build(&self) -> String {
        match self {
            Scoring::Bm25(k1, b) => format!("BM25 {} {}", k1.build(), b.build()),
            Scoring::Vs => "VS".to_string(),
        }
    }
}

impl Parametric for Scoring {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Scoring::Bm25(k1, b) => [k1.get_bindings(), b.get_bindings()].concat(),
            Scoring::Vs => vec![],
        }
    }
}

impl Erroneous for Scoring {
    fn get_errors(&self) -> crate::ErrorList {
        match self {
            Scoring::Bm25(k1, b) => [k1.get_errors(), b.get_errors()].concat(),
            Scoring::Vs => vec![],
        }
    }
}

// Builder for SearchParams
pub struct SearchAnalyzer {
    anayzer: Option<TableLike>,
    highlight: Option<bool>,
    scoring: Option<Scoring>,
    doc_ids_order: Option<NumberLike>,
    doc_lengths_order: Option<NumberLike>,
    postings_order: Option<NumberLike>,
    terms_order: Option<NumberLike>,
}

impl SearchAnalyzer {
    pub fn highlight(mut self) -> Self {
        self.highlight = Some(true);
        self
    }

    // pub fn scoring(mut self, scoring: impl Into<Scoring>) -> Self {
    //     self.scoring = Some(scoring.into());
    //     self
    // }

    pub fn bm25(mut self, k1: impl Into<NumberLike>, b: impl Into<NumberLike>) -> Self {
        self.scoring = Some(Scoring::Bm25(k1.into(), b.into()));
        self
    }

    pub fn vs(mut self) -> Self {
        self.scoring = Some(Scoring::Vs);
        self
    }

    pub fn doc_ids_order(mut self, doc_ids_order: impl Into<NumberLike>) -> Self {
        self.doc_ids_order = Some(doc_ids_order.into());
        self
    }

    pub fn doc_lengths_order(mut self, doc_lengths_order: impl Into<NumberLike>) -> Self {
        self.doc_lengths_order = Some(doc_lengths_order.into());
        self
    }

    pub fn postings_order(mut self, postings_order: impl Into<NumberLike>) -> Self {
        self.postings_order = Some(postings_order.into());
        self
    }

    pub fn terms_order(mut self, terms_order: impl Into<NumberLike>) -> Self {
        self.terms_order = Some(terms_order.into());
        self
    }
}

impl Parametric for SearchAnalyzer {
    fn get_bindings(&self) -> BindingsList {
        let mut bindings = vec![];
        if let Some(az) = &self.anayzer {
            bindings.extend(az.get_bindings());
        }

        if let Some(sc) = &self.scoring {
            bindings.extend(sc.get_bindings());
        }

        if let Some(doc_ids_order) = &self.doc_ids_order {
            bindings.extend(doc_ids_order.get_bindings());
        }

        if let Some(doc_lengths_order) = &self.doc_lengths_order {
            bindings.extend(doc_lengths_order.get_bindings());
        }

        if let Some(postings_order) = &self.postings_order {
            bindings.extend(postings_order.get_bindings());
        }

        if let Some(terms_order) = &self.terms_order {
            bindings.extend(terms_order.get_bindings());
        }

        bindings
    }
}

impl Erroneous for SearchAnalyzer {
    fn get_errors(&self) -> crate::ErrorList {
        let mut errors = vec![];
        if let Some(az) = &self.anayzer {
            errors.extend(az.get_errors());
        }

        if let Some(sc) = &self.scoring {
            errors.extend(sc.get_errors());
        }

        if let Some(doc_ids_order) = &self.doc_ids_order {
            errors.extend(doc_ids_order.get_errors());
        }

        if let Some(doc_lengths_order) = &self.doc_lengths_order {
            errors.extend(doc_lengths_order.get_errors());
        }

        if let Some(postings_order) = &self.postings_order {
            errors.extend(postings_order.get_errors());
        }

        if let Some(terms_order) = &self.terms_order {
            errors.extend(terms_order.get_errors());
        }

        errors
    }
}

// use .build method on each field
// format!("{}", az.build())
impl Buildable for SearchAnalyzer {
    fn build(&self) -> String {
        let mut query = String::new();
        if let Some(az) = &self.anayzer {
            query = format!("{query}SEARCH ANALYZER {}", az.build());
        }

        if let Some(true) = &self.highlight {
            query = format!("{query} HIGHLIGHTS");
        }

        if let Some(sc) = &self.scoring {
            query = format!("{query} {}", sc.build());
        }

        if let Some(doc_ids_order) = &self.doc_ids_order {
            query = format!("{query} DOC_IDS_ORDER {}", doc_ids_order.build());
        }

        if let Some(doc_lengths_order) = &self.doc_lengths_order {
            query = format!("{query} DOC_LENGTHS_ORDER {}", doc_lengths_order.build());
        }

        if let Some(postings_order) = &self.postings_order {
            query = format!("{query} POSTINGS_ORDER {}", postings_order.build());
        }

        if let Some(terms_order) = &self.terms_order {
            query = format!("{query} TERMS_ORDER {}", terms_order.build());
        }

        query
    }
}

/// Function to start building a SearchParams object with an analyzer
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{statements::search_analyzer};
///
/// search_analyzer("my_analyzer")
///    .highlight()
///    .bm25(1.2, 0.75)
///    .doc_ids_order(1)
///    .doc_lengths_order(1)
///    .postings_order(1)
///    .terms_order(1);
/// ```
pub fn search_analyzer(analyzer_name: impl Into<TableLike>) -> SearchAnalyzer {
    SearchAnalyzer {
        anayzer: Some(analyzer_name.into()),
        highlight: None,
        scoring: None,
        doc_ids_order: None,
        doc_lengths_order: None,
        postings_order: None,
        terms_order: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{arr, ToRaw};

    use super::*;

    #[test]
    fn define_index_with_search_analyzer() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields([email])
            .search_analyzer(
                search_analyzer("ascii")
                    .highlight()
                    .bm25(1.2, 0.75)
                    .doc_ids_order(1)
                    .doc_lengths_order(1)
                    .postings_order(1)
                    .terms_order(1),
            );

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email SEARCH ANALYZER ascii HIGHLIGHTS BM25 1.2f 0.75f DOC_IDS_ORDER 1 DOC_LENGTHS_ORDER 1 POSTINGS_ORDER 1 TERMS_ORDER 1;"
        );
        assert_eq!(query.fine_tune_params(),
        "DEFINE INDEX userEmailIndex ON TABLE $_param_00000001 FIELDS email SEARCH ANALYZER $_param_00000002 HIGHLIGHTS BM25 $_param_00000003 $_param_00000004 DOC_IDS_ORDER $_param_00000005 DOC_LENGTHS_ORDER $_param_00000006 POSTINGS_ORDER $_param_00000007 TERMS_ORDER $_param_00000008;"
        );
        assert_eq!(query.get_bindings().len(), 8);
    }

    #[test]
    fn test_define_index_statement_single_field() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields(email)
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_single_column() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .columns(email)
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_multiple_fields() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .fields(arr![age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_multiple_columns() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .columns([age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX alien_index ON TABLE alien COLUMNS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }
}
