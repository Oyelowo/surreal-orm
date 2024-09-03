/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::ops::Deref;

use crate::{
    statements::{LetStatement, Subquery},
    Arrow, Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList, Model, Operatable,
    Operation, Param, Parametric, Setter, Table,
};

use super::{Filter, NumberLike, SurrealId};

/// Use to generate a filter for a query. Can be used as part of a statement
/// or filtering of a node or edge.
pub fn where_(condition: impl Conditional) -> Filter {
    if condition.get_errors().is_empty() {
        // TODO: Maybe pass to DB filter and check and return Result<Filter> in relate_query
    }
    Filter::new(condition)
}

/// Used as argument in node and edges for filtering or selecting.
#[derive(Debug, Clone)]
pub enum ClauseType {
    /// Stands for `*`
    All,
    /// Stands for `$`
    Last,
    /// Stands for a specific index value e.g `[3]`
    Index(Index),
    /// No space
    Empty,
    /// Stands for a WHERE clause e.g `WHERE id = 1`
    Where(Filter),
    /// Stands for a full Select statement
    Subquery(Subquery),
    /// Stands for a field e.g `person:oyelowo`. This is useful in a RELATE query.
    Id(sql::Thing),
    /// Stands for a parameter e.g `$name`
    Param(Param),
    /// Used for filtering on multiple edges
    AnyEdgeFilter(AnyEdgeFilter),
}

#[derive(Debug, Clone)]
enum ModelOrFieldName {
    Model(String),
    Field(String),
}

/// Contains metadata for Array Clause
#[derive(Debug, Clone)]
pub struct Clause {
    kind: ClauseType,
    arrow: Option<Arrow>,
    model_or_field_name: Option<ModelOrFieldName>,
    query_string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Buildable for Clause {
    fn build(&self) -> String {
        let connection_name = match self.model_or_field_name.clone() {
            Some(name) => match name {
                ModelOrFieldName::Model(m) => m,
                ModelOrFieldName::Field(f) => f,
            },
            None => "".to_string(),
        };

        let clause = match self.kind.clone() {
            ClauseType::Subquery(subquery) => subquery.build(),
            // ClauseType::SelectStatement(_q) => self.clone().query_string,
            ClauseType::Param(p) => p.build(),
            ClauseType::AnyEdgeFilter(edge_filters) => {
                format!("({}, {})", connection_name, edge_filters.build(),)
            }
            ClauseType::Id(_id) => self
                .get_bindings()
                .pop()
                .expect("Id must have only one binding. Has to be an error. Please report.")
                .get_param_dollarised()
                .to_string(),
            _ => format!("{}{}", connection_name, self.query_string),
        };

        let connection = self
            .arrow
            .as_ref()
            .map_or(clause.clone(), |a| format!("{}{}", &a, clause));
        connection
    }
}

/// Clause for a Node alias
#[derive(Debug, Clone)]
pub struct NodeAliasClause(NodeClause);

impl NodeAliasClause {
    /// access wrapped NodeClause
    pub fn into_inner(self) -> NodeClause {
        self.0
    }
}

impl<T> From<T> for NodeAliasClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(NodeClause(clause))
    }
}

/// Clause for Node
#[derive(Debug, Clone)]
pub struct NodeClause(Clause);

impl Parametric for NodeClause {
    fn get_bindings(&self) -> BindingsList {
        self.0.get_bindings()
    }
}

impl Buildable for NodeClause {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl Erroneous for NodeClause {
    fn get_errors(&self) -> ErrorList {
        self.0.get_errors()
    }
}

impl NodeClause {
    /// Create a new NodeClause with arrow. This is used in the macro for building a graph query.
    /// Sometimes, nodes need to be appended with arrow if buinding e.g node->edge->node
    pub fn with_arrow(self, arrow: impl Into<Arrow>) -> Self {
        Self(self.0.with_arrow(arrow))
    }

    /// attach the table name to the clause as metadata. Useful for doing some checks.
    pub fn with_table(self, table: impl Into<String>) -> Self {
        Self(self.0.with_table(table))
    }

    /// attach the field name to the clause as metadata.
    pub fn with_field(self, field_name: String) -> Self {
        Self(self.0.with_field(field_name))
    }
}

impl<T, Id> From<SurrealId<T, Id>> for NodeClause
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: SurrealId<T, Id>) -> Self {
        Self(Clause::new(ClauseType::Id(value.to_thing())))
    }
}

impl<T, Id> From<&SurrealId<T, Id>> for NodeClause
where
    T: Model,
    Id: Into<sql::Id>,
{
    fn from(value: &SurrealId<T, Id>) -> Self {
        Self(Clause::new(ClauseType::Id(value.to_thing())))
    }
}

impl From<sql::Thing> for NodeClause {
    fn from(value: sql::Thing) -> Self {
        Self(Clause::new(ClauseType::Id(value)))
    }
}

impl From<&sql::Thing> for NodeClause {
    fn from(value: &sql::Thing) -> Self {
        Self(Clause::new(ClauseType::Id(value.clone())))
    }
}

impl From<Param> for NodeClause {
    fn from(value: Param) -> Self {
        Self(Clause::new(ClauseType::Param(value.clone())))
    }
}

impl From<LetStatement> for NodeClause {
    fn from(value: LetStatement) -> Self {
        Self(Clause::new(ClauseType::Param(value.get_param())))
    }
}
// impl From<Param> for Clause {
//     fn from(value: Param) -> Self {
//         Clause::new(ClauseType::Param(value.clone()))
//     }
// }
//
// impl From<LetStatement> for Clause {
//     fn from(value: LetStatement) -> Self {
//         Clause::new(ClauseType::Param(value.get_param()))
//     }
// }

impl<T> From<T> for NodeClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(clause)
    }
}

/// Clause for an Edge
#[derive(Debug, Clone)]
pub struct EdgeClause(Clause);

impl Parametric for EdgeClause {
    fn get_bindings(&self) -> BindingsList {
        self.0.get_bindings()
    }
}

impl Buildable for EdgeClause {
    fn build(&self) -> String {
        self.0.build()
    }
}
impl Erroneous for EdgeClause {
    fn get_errors(&self) -> ErrorList {
        self.0.get_errors()
    }
}

impl EdgeClause {
    /// Create a new EdgeClause with arrow. This is used in the macro for building a graph query.
    pub fn with_arrow(self, arrow: impl Into<Arrow>) -> Self {
        Self(self.0.with_arrow(arrow))
    }

    /// attach the table name to the clause as metadata. Useful for doing some checks.
    pub fn with_table(self, table: impl Into<String>) -> Self {
        Self(self.0.with_table(table))
    }
}

impl<T> From<T> for EdgeClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(clause)
    }
}

/// Clause for an Object
#[derive(Debug, Clone)]
pub struct ObjectClause(Clause);

impl ObjectClause {
    /// Create a new ObjectClause with arrow. This is used in the macro for building a graph
    /// query.
    pub fn with_arrow(self, arrow: impl Into<Arrow>) -> Self {
        Self(self.0.with_arrow(arrow))
    }

    /// attach the table name to the clause as metadata. Useful for doing some checks.
    pub fn with_table(self, table: &str) -> Self {
        Self(self.0.with_table(table))
    }

    /// attach the field name to the clause as metadata.
    pub fn with_field(self, field_name: impl Into<String>) -> Self {
        Self(self.0.with_field(field_name))
    }
}

impl<T> From<T> for ObjectClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(clause)
    }
}

impl Parametric for ObjectClause {
    fn get_bindings(&self) -> BindingsList {
        self.0.get_bindings()
    }
}

impl Buildable for ObjectClause {
    fn build(&self) -> String {
        self.0.build()
    }
}

impl Erroneous for ObjectClause {
    fn get_errors(&self) -> ErrorList {
        self.0.get_errors()
    }
}

impl From<Operation> for Clause {
    fn from(value: Operation) -> Self {
        let filter = Filter::new(value);
        Self::new(ClauseType::Where(filter))
    }
}

impl From<Setter> for Clause {
    fn from(value: Setter) -> Self {
        let filter: Operation = value.into();
        let filter = Filter::new(filter);
        Self::new(ClauseType::Where(filter))
    }
}

impl From<&Self> for Clause {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl Parametric for Clause {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for Clause {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Clause {
    /// Create a new Clause
    pub fn new(kind: ClauseType) -> Self {
        use ClauseType::*;
        let mut bindings = vec![];
        let mut errors = vec![];

        let query_string = match &kind {
            Empty => "".into(),
            Where(filter) => {
                bindings = filter.get_bindings();
                errors = filter.get_errors();
                format!("[WHERE {filter}]")
            }
            Param(param) => {
                bindings = param.get_bindings();
                errors = param.get_errors();
                param.build().to_string()
            }
            Id(surreal_id) => {
                // The Table name component of the Id comes from the macro. e.g For student:5, the Schema which this is wrapped into provide. So all we need here is the id component, student
                let id_bindings = Binding::new(surreal_id.clone());
                let param_string = id_bindings.get_param_dollarised().to_string();
                errors = vec![];
                bindings = vec![id_bindings];
                param_string
            }
            Subquery(subquery) => {
                bindings = subquery.get_bindings();
                errors = subquery.get_errors();
                subquery.build()
            }
            All => "[*]".to_string(),
            Last => "[$]".to_string(),
            Index(index) => {
                bindings = index.get_bindings();
                format!("[{}]", index.build())
            }
            AnyEdgeFilter(edge_tables) => {
                bindings = edge_tables.get_bindings();
                errors = edge_tables.get_errors();
                let build = edge_tables.build().to_string();
                format!("({build})")
            }
        };

        Self {
            kind,
            query_string,
            bindings,
            arrow: None,
            model_or_field_name: None,
            errors,
        }
    }

    /// Create a new Clause with arrow. This is used in the macro for building a graph query.
    pub fn with_arrow(mut self, arrow: impl Into<Arrow>) -> Self {
        self.arrow = Some(arrow.into());
        self
    }

    /// attach the table name to the clause as metadata. Useful for doing some checks.
    pub fn with_table(self, table: impl Into<String>) -> Self {
        let table: String = table.into();
        let mut updated_clause = self.update_errors(&table);
        updated_clause.model_or_field_name = Some(ModelOrFieldName::Model(table));
        updated_clause
    }

    /// attach the field name to the clause as metadata.
    pub fn with_field(mut self, field_name: impl Into<String>) -> Self {
        let field_name: String = field_name.into();
        self.model_or_field_name = Some(ModelOrFieldName::Field(field_name));
        self
    }

    fn update_errors(mut self, table: &str) -> Self {
        let mut errors = vec![];
        if let ClauseType::Id(id) = &self.kind {
            if !id.to_string().starts_with(format!("{table}:").as_str()) {
                errors.push(format!(
                    "invalid id {id}. Id does not belong to table {table}"
                ))
            }
        }
        self.errors = errors;
        self
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl From<Filter> for Clause {
    fn from(value: Filter) -> Self {
        Self::new(ClauseType::Where(value))
    }
}

impl From<&Filter> for Clause {
    fn from(value: &Filter) -> Self {
        Self::new(ClauseType::Where(value.clone()))
    }
}

impl From<Empty> for Clause {
    fn from(_value: Empty) -> Self {
        Self::new(ClauseType::Empty)
    }
}

impl<T: Into<Subquery>> From<T> for Clause {
    fn from(value: T) -> Self {
        let value: Subquery = value.into();
        Self::new(ClauseType::Subquery(value))
    }
}

// impl<T: Into<Subquery>> From<&T> for Clause {
//     fn from(value: &T) -> Self {
//         let value: Subquery = value.into();
//         Self::new(ClauseType::Subquery(value))
//     }
// }
// impl From<&SelectStatement> for Clause {
//     fn from(value: &SelectStatement) -> Self {
//         // Self::Query(value.to_owned().into())
//         Self::new(ClauseType::SelectStatement(value.clone()))
//     }
// }

// impl From<SelectStatement> for Clause {
//     fn from(value: SelectStatement) -> Self {
//         Self::new(ClauseType::SelectStatement(value))
//     }
// }
//
// impl From<&SelectStatement> for Clause {
//     fn from(value: &SelectStatement) -> Self {
//         // Self::Query(value.to_owned().into())
//         Self::new(ClauseType::SelectStatement(value.clone()))
//     }
// }

/// Use when you want no space. Also aliased as `E`.
pub struct Empty;
use surrealdb::sql;
pub use Empty as E;

impl Operatable for Empty {}

impl Buildable for Empty {
    fn build(&self) -> String {
        "".to_string()
    }
}

impl Conditional for Empty {
    fn get_condition_query_string(&self) -> String {
        "".to_string()
    }
}

impl Erroneous for Empty {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Parametric for Empty {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

/// stands for `*`
pub struct All;

impl From<All> for Clause {
    fn from(_value: All) -> Self {
        Self::new(ClauseType::All)
    }
}

impl From<Last> for Clause {
    fn from(_value: Last) -> Self {
        Self::new(ClauseType::Last)
    }
}

impl std::fmt::Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}

/// stands for `$`
pub struct Last;

impl std::fmt::Display for Last {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$")
    }
}

/// stands for index number in a list e.g `[3]`
#[derive(Debug, Clone)]
pub struct Index(NumberLike);

impl Deref for Index {
    type Target = NumberLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Index> for Clause {
    fn from(value: Index) -> Self {
        Self::new(ClauseType::Index(value))
    }
}

impl From<[u128; 1]> for Clause {
    fn from(value: [u128; 1]) -> Self {
        index(value[0]).into()
    }
}

/// Create an index
pub fn index(index: impl Into<NumberLike>) -> Index {
    Index(index.into())
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.build())
    }
}

/// Creates WHERE filter for one or more edges
#[derive(Debug, Clone)]
pub struct AnyEdgeFilter {
    edge_tables: Vec<Table>,
    where_: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Erroneous for AnyEdgeFilter {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl AnyEdgeFilter {
    /// Creates a WHERE condition for one or more edges
    /// # Example
    /// ```rust, ignore
    /// Student::schema()
    ///             .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
    ///             .book(Empty);
    /// ```
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.bindings.extend(condition.get_bindings());
        self.errors.extend(condition.get_errors());

        let condition = Filter::new(condition);
        self.where_ = Some(condition.build());
        self
    }
}

impl Buildable for AnyEdgeFilter {
    fn build(&self) -> String {
        let mut query = format!(
            "{} ",
            self.edge_tables
                .to_vec()
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        if let Some(where_) = &self.where_ {
            query = format!("{} WHERE {}", query, where_);
        }

        query
    }
}

impl Parametric for AnyEdgeFilter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<AnyEdgeFilter> for EdgeClause {
    fn from(value: AnyEdgeFilter) -> Self {
        Self(Clause::new(ClauseType::AnyEdgeFilter(value)))
    }
}

/// Creates WHERE filter for one or more edges. This allows for pattern like this:
/// ```sql
/// Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
/// ```
/// SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;
///
/// Examples:
///
/// ```rust, ignore
/// # let student_id = SurrealId::try_from("student:1").unwrap();
/// # let book_id = SurrealId::try_from("book:2").unwrap();
/// # let likes = Table::new("likes");
/// # let writes = StudentWritesBook::table();
/// # let timeWritten = Field::new("timeWritten");
///  Student::with(student_id)
///     .writes__(Empty)
///     .writes__(Empty)
///     .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
///     .book(book_id)
///     .__as__(Student::aliases().writtenBooks);
/// ```
pub fn any_other_edges(edges: impl Into<crate::Tables>) -> AnyEdgeFilter {
    AnyEdgeFilter {
        edge_tables: edges.into().into(),
        where_: None,
        bindings: vec![],
        errors: vec![],
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cond,
        statements::{let_, select},
        Field, Param, ToRaw,
    };

    use super::*;

    #[test]
    fn test_display_clause_with_empty() {
        let empty_clause = Clause::from(Empty);
        assert_eq!(empty_clause.build(), "");
        assert_eq!(empty_clause.to_raw().build(), "");
    }

    #[test]
    fn test_display_clause_with_where_filter() {
        let filter = cond(Field::new("age").equal(18));
        let where_clause = Clause::from(filter);
        assert_eq!(
            where_clause.fine_tune_params(),
            "[WHERE age = $_param_00000001]"
        );
        assert_eq!(where_clause.to_raw().build(), "[WHERE age = 18]");
    }

    #[test]
    fn test_display_clause_with_id_only_wors_with_node() {
        let id_clause =
            NodeClause::from(sql::Thing::from(("student".to_string(), "oye".to_string())));
        assert_eq!(id_clause.fine_tune_params(), "$_param_00000001");
        assert_eq!(id_clause.to_raw().build(), "student:oye");
    }

    #[test]
    fn test_display_clause_with_query() {
        let table = Table::new("students");
        let select_statement = select(All).from(table);
        let query_clause = Clause::from(select_statement);
        assert_eq!(query_clause.fine_tune_params(), "$_param_00000001");
        assert_eq!(query_clause.get_bindings().len(), 1);
        assert_eq!(query_clause.get_errors().len(), 0);
        assert_eq!(query_clause.to_raw().build(), "(SELECT * FROM students)");
    }

    #[test]
    fn test_display_clause_with_param() {
        let table = Table::new("students");
        let devs = let_("devs").equal_to(select(All).from(table));
        let clause = NodeClause::from(devs.get_param());
        assert_eq!(clause.build(), "$devs");
        assert_eq!(clause.to_raw().build(), "$devs");
    }

    #[test]
    fn test_display_clause_with_let_statement() {
        let table = Table::new("students");
        let devs = let_("devs").equal_to(select(All).from(table));
        let clause = NodeClause::from(devs);
        assert_eq!(clause.build(), "$devs");
        assert_eq!(clause.to_raw().build(), "$devs");
    }

    #[test]
    fn test_display_clause_with_all() {
        let all_clause = Clause::from(All);
        assert_eq!(all_clause.build(), "[*]");
        assert_eq!(all_clause.to_raw().build(), "[*]");
    }

    #[test]
    fn test_display_clause_with_last() {
        let last_clause = Clause::from(Last);
        assert_eq!(last_clause.build(), "[$]");
        assert_eq!(last_clause.to_raw().build(), "[$]");
    }

    #[test]
    fn test_display_clause_with_index() {
        let index_clause = Clause::from(index(42));
        assert_eq!(index_clause.fine_tune_params(), "[$_param_00000001]");
        assert_eq!(index_clause.to_raw().build(), "[42]");
    }

    #[test]
    fn test_display_clause_with_index_field() {
        let position = Field::new("position");
        let index_clause = Clause::from(index(position));
        assert_eq!(index_clause.fine_tune_params(), "[position]");
        assert_eq!(index_clause.to_raw().build(), "[position]");
    }

    #[test]
    fn test_display_clause_with_index_param() {
        let position = Param::new("position");
        let index_clause = Clause::from(index(position));
        assert_eq!(index_clause.fine_tune_params(), "[$position]");
        assert_eq!(index_clause.to_raw().build(), "[$position]");
    }

    #[test]
    fn test_display_clause_with_any_edge_condition_simple() {
        let writes = Table::new("writes");
        let reads = Table::new("reads");
        let purchased = Table::new("purchased");
        let amount = Field::new("amount");

        let age_edge_condition =
            any_other_edges(vec![writes, reads, purchased]).where_(amount.less_than_or_equal(120));

        assert_eq!(
            age_edge_condition.fine_tune_params(),
            "writes, reads, purchased  WHERE amount <= $_param_00000001"
        );
        assert_eq!(
            age_edge_condition.to_raw().build(),
            "writes, reads, purchased  WHERE amount <= 120"
        );
    }

    #[test]
    fn test_display_clause_with_any_edge_condition_complex() {
        let writes = Table::new("writes");
        let reads = Table::new("reads");
        let purchased = Table::new("purchased");
        let city = Field::new("city");

        let age_edge_condition = any_other_edges(vec![writes, reads, purchased]).where_(
            cond(city.is("Prince Edward Island"))
                .and(city.is("NewFoundland"))
                .or(city.like("Toronto")),
        );

        assert_eq!(
        age_edge_condition.fine_tune_params(),
        "writes, reads, purchased  WHERE (city IS $_param_00000001) AND (city IS $_param_00000002) OR (city ~ $_param_00000003)"
    );
        assert_eq!(
         age_edge_condition.to_raw().build(),
        "writes, reads, purchased  WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR (city ~ 'Toronto')"
    );
    }
}
