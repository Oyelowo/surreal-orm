use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;

// use crate::prelude::SqlSerializeResult;

type CowSegment<'a> = Cow<'a, str>;

pub struct QueryBuilder<'a> {
    segments: Vec<CowSegment<'a>>,
    parameters: HashMap<&'a str, &'a str>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new() -> Self {
        QueryBuilder {
            segments: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().create("Person:ee").build();
    ///
    /// assert_eq!(query, "CREATE Person:ee")
    /// ```
    pub fn create<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("CREATE", node);

        self
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().update("Person:ee").build();
    ///
    /// assert_eq!(query, "UPDATE Person:ee")
    /// ```
    pub fn update<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("UPDATE", node);

        self
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().select("ee:Person").build();
    ///
    /// assert_eq!(query, "SELECT ee:Person")
    /// ```
    pub fn select<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("SELECT", node);

        self
    }

    /// Start a `DELETE` statement:
    /// ```sql
    /// DELETE user:John
    /// ```
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().delete("ee:Person").build();
    ///
    /// assert_eq!(query, "DELETE ee:Person");
    /// ```
    pub fn delete<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("DELETE", node);

        self
    }

    /// Start a `RELATE` statement:
    /// ```sql
    /// RELATE user:tobie->write->article:surreal SET time.written = time::now();
    /// ```
    /// _Note: the `SET` or anything after it in the example above should be added
    /// manually using the [`QueryBuilder::set()`] method._
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().relate("user:John->likes->user:Mark").build();
    ///
    /// assert_eq!(query, "RELATE user:John->likes->user:Mark");
    /// ```
    pub fn relate<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("RELATE", node);

        self
    }

    /// Start a `CONTENT` statement. Content statements often follow RELATE statements:
    /// ```sql
    /// RELATE user:tobie->write->article:surreal CONTENT {
    ///   source: 'Apple notes',
    ///   tags: ['notes', 'markdown'],
    ///   time: {
    ///     written: time::now(),
    ///   },
    /// };
    /// ```
    /// _Note: Anything before the `CONTENT` in the example above should be added
    /// manually using the [`QueryBuilder::relate()`] method._
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().content("{ creation_time: time::now() }").build();
    ///
    /// assert_eq!(query, "CONTENT { creation_time: time::now() }");
    /// ```
    pub fn content<T: Into<CowSegment<'a>>>(mut self, json_content: T) -> Self {
        self.add_segment_p("CONTENT", json_content);

        self
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().from("Person").build();
    ///
    /// assert_eq!(query, "FROM Person")
    pub fn from<T: Into<CowSegment<'a>>>(mut self, node: T) -> Self {
        self.add_segment_p("FROM", node);

        self
    }

    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().select_many(&["ee:Person", "o:Order"]).build();
    ///
    /// assert_eq!(query, "SELECT ee:Person , o:Order")
    /// ```
    pub fn select_many<T: Into<CowSegment<'a>>>(mut self, nodes: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("SELECT");
        self.join_segments(",", "", nodes, "");

        self
    }

    /// Adds the supplied query with a comma in front of it
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new().also("ee").build();
    ///
    /// assert_eq!(query, ", ee")
    /// ```
    pub fn also<T: Into<CowSegment<'a>>>(mut self, query: T) -> Self {
        self.add_segment_p(",", query);

        self
    }

    /// Adds the given segments, separated by the given `separator` and with a `prefix`
    /// and a `suffix` added to them too.
    ///
    /// # Example
    /// ```rs
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .join_segments(",", "set", &["handle", "id"], "")
    ///   .build();
    ///
    /// assert_eq!(query, "set handle , set id");
    /// ```
    #[allow(dead_code)]
    fn join_segments<T: Into<CowSegment<'a>>>(
        &mut self,
        seperator: &'a str,
        prefix: &'a str,
        segments: &[T],
        suffix: &'a str,
    ) -> &mut Self
    where
        T: Copy,
    {
        let segments_count = segments.len();

        if segments_count <= 1 {
            for segment in segments {
                self.add_segment_ps(prefix, *segment, suffix);
            }

            return self;
        }

        for i in 0..segments_count - 1 {
            self.add_segment_ps(prefix, segments[i], suffix);
            self.add_segment(seperator);
        }

        self.add_segment_ps(prefix, segments[segments_count - 1], suffix);

        self
    }

    /// Starts a WHERE clause.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .filter("handle = ?1")
    ///   .build();
    ///
    /// assert_eq!(query, "WHERE handle = ?1");
    /// ```
    pub fn filter<T: Into<CowSegment<'a>>>(mut self, condition: T) -> Self {
        self.add_segment_p("WHERE", condition);

        self
    }

    /// An alias for `QueryBuilder::filter`
    pub fn and_where<T: Into<CowSegment<'a>>>(self, condition: T) -> Self {
        self.filter(condition)
    }

    /// Writes a OR followed by the supplied `condition`
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .or("handle = ?1")
    ///   .build();
    ///
    /// assert_eq!(query, "OR handle = ?1");
    /// ```
    pub fn or<T: Into<CowSegment<'a>>>(mut self, condition: T) -> Self {
        self.add_segment_p("OR", condition);

        self
    }

    /// Starts an AND followed by the supplied `condition`.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .and("handle = ?1")
    ///   .build();
    ///
    /// assert_eq!(query, "AND handle = ?1");
    /// ```
    pub fn and<T: Into<CowSegment<'a>>>(mut self, condition: T) -> Self {
        self.add_segment_p("AND", condition);

        self
    }

    /// Starts a SET clause.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .set("handle = ?1")
    ///   .build();
    ///
    /// assert_eq!(query, "SET handle = ?1");
    /// ```
    pub fn set<T: Into<CowSegment<'a>>>(mut self, update: T) -> Self {
        self.add_segment_p("SET", update);

        self
    }

    /// Starts a SET clause with many fields.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .set_many(&["handle = $1", "password = $2"])
    ///   .build();
    ///
    /// assert_eq!(query, "SET handle = $1 , password = $2");
    /// ```
    pub fn set_many<T: Into<CowSegment<'a>>>(mut self, updates: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("SET");
        self.join_segments(",", "", updates, "");

        self
    }

    /// Starts a FETCH clause,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .fetch("author")
    ///   .build();
    ///
    /// assert_eq!(query, "FETCH author");
    /// ```
    pub fn fetch<T: Into<CowSegment<'a>>>(mut self, field: T) -> Self {
        self.add_segment_p("FETCH", field);

        self
    }

    /// Starts a FETCH clause with zero or more fields,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .fetch_many(&["author", "projects"])
    ///   .build();
    ///
    /// assert_eq!(query, "FETCH author , projects");
    /// ```
    pub fn fetch_many<T: Into<CowSegment<'a>>>(mut self, fields: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("FETCH");
        self.join_segments(",", "", fields, "");

        self
    }

    /// Starts a GROUP BY clause,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .group_by("author")
    ///   .build();
    ///
    /// assert_eq!(query, "GROUP BY author");
    /// ```
    pub fn group_by<T: Into<CowSegment<'a>>>(mut self, field: T) -> Self {
        self.add_segment_p("GROUP BY", field);

        self
    }

    /// Starts a GROUP BY clause with zero or more fields,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .group_by_many(&["author", "projects"])
    ///   .build();
    ///
    /// assert_eq!(query, "GROUP BY author , projects");
    /// ```
    pub fn group_by_many<T: Into<CowSegment<'a>>>(mut self, fields: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("GROUP BY");
        self.join_segments(",", "", fields, "");

        self
    }

    /// Starts a ORDER BY ASC clause,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .order_by_asc("author")
    ///   .build();
    ///
    /// assert_eq!(query, "ORDER BY author ASC");
    /// ```
    pub fn order_by_asc<T: Into<CowSegment<'a>>>(mut self, field: T) -> Self {
        self.add_segment_ps("ORDER BY", field, "ASC");

        self
    }

    /// Starts a ORDER BY ASC clause with zero or more fields,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .order_by_asc_many(&["author", "projects"])
    ///   .build();
    ///
    /// assert_eq!(query, "ORDER BY author ASC , projects ASC");
    /// ```
    pub fn order_by_asc_many<T: Into<CowSegment<'a>>>(mut self, fields: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("ORDER BY");
        self.join_segments(",", "", fields, "ASC");

        self
    }

    /// Starts a ORDER BY DESC clause,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .order_by_desc("author")
    ///   .build();
    ///
    /// assert_eq!(query, "ORDER BY author DESC");
    /// ```
    pub fn order_by_desc<T: Into<CowSegment<'a>>>(mut self, field: T) -> Self {
        self.add_segment_ps("ORDER BY", field, "DESC");

        self
    }

    /// Starts a ORDER BY DESC clause with zero or more fields,
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .order_by_desc_many(&["author", "projects"])
    ///   .build();
    ///
    /// assert_eq!(query, "ORDER BY author DESC , projects DESC");
    /// ```
    pub fn order_by_desc_many<T: Into<CowSegment<'a>>>(mut self, fields: &[T]) -> Self
    where
        T: Copy,
    {
        self.add_segment("ORDER BY");
        self.join_segments(",", "", fields, "DESC");

        self
    }

    /// Queues a condition which allows the next statement to be ignored if
    /// `condition` is `false`.
    ///
    /// Conditions can be nested, the queue works as a LIFO queue.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .select_many(&["1", "2"])
    ///   .if_then(false, |query| query
    ///     .select_many(&["3", "4"])
    ///     // will not run:
    ///     .if_then(true, |query| query
    ///       .select_many(&["5", "6"])
    ///     )
    ///   )
    ///   .if_then(true, |query| query
    ///     .select_many(&["7", "8"])
    ///   )
    ///   .build();
    ///
    /// assert_eq!(query, "SELECT 1 , 2 SELECT 7 , 8");
    /// ```
    pub fn if_then<F>(self, condition: bool, action: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if !condition {
            return self;
        }

        action(self)
    }

    /// Writes an AND followed by the supplied `first_condition` and any other
    /// statement added to the querybuilder in the `action` closure surrounded by
    /// parenthesis.
    ///
    /// Can be used to compose boolean expressions with grouped OR statements like so:
    /// ```sql
    /// WHERE name contains 'John' AND (name contains 'Doe' OR name contains 'Eod')
    /// ```
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .select("*")
    ///   .from("user")
    ///   .filter("name contains 'John'")
    ///   .and_group("name contains 'Doe'", |q| {
    ///     q.or("name contains 'Eod'")
    ///   })
    ///   .build();
    ///
    /// assert_eq!(query, "SELECT * FROM user WHERE name contains 'John' AND ( name contains 'Doe' OR name contains 'Eod' )");
    /// ```
    pub fn and_group<F, T: Into<CowSegment<'a>>>(mut self, first_condition: T, action: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        self.add_segment_p("AND", "(");
        self.add_segment(first_condition);
        let mut output = action(self);
        output.add_segment(")");

        output
    }

    /// Pushes raw text to the buffer
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .raw("foo bar")
    ///   .build();
    ///
    /// assert_eq!(query, "foo bar");
    /// ```
    pub fn raw(mut self, text: &'a str) -> Self {
        self.add_segment(text);

        self
    }

    /// Start a queue where all of the new pushed actions are separated by commas.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .commas(|query| query
    ///     .raw("foo")
    ///     .raw("bar")
    ///   ).build();
    ///
    /// assert_eq!(query, "foo , bar");
    /// ```
    pub fn commas(mut self, action: fn(Self) -> Self) -> Self {
        let other = action(QueryBuilder::new());

        for (index, segment) in other.segments.into_iter().enumerate() {
            if index <= 0 {
                self.segments.push(segment);
            } else {
                self.add_segment(",");
                self.segments.push(segment);
            }
        }

        self
    }

    /// Start a LIMIT clause.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    ///
    /// let page_size = 10.to_string();
    /// let query = QueryBuilder::new()
    ///   .limit(&page_size)
    ///   .build();
    ///
    /// assert_eq!(query, "LIMIT 10")
    ///
    /// ```
    pub fn limit<T: Into<CowSegment<'a>>>(mut self, limit: T) -> Self {
        self.add_segment_p("LIMIT", limit);

        self
    }

    /// Start a START AT clause.
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    ///
    /// let page_size = 10.to_string();
    /// let query = QueryBuilder::new()
    ///   .start_at(&page_size)
    ///   .build();
    ///
    /// assert_eq!(query, "START AT 10")
    ///
    /// ```
    pub fn start_at<T: Into<CowSegment<'a>>>(mut self, offset: T) -> Self {
        self.add_segment_p("START AT", offset);

        self
    }

    /// Add the given segment to the internal buffer. This is a rather internal
    /// method that is set public for special cases, you should prefer using the `raw`
    /// method instead.
    pub fn add_segment<T: Into<CowSegment<'a>>>(&mut self, segment: T) -> &mut Self {
        let into = segment.into();

        if into.is_empty() {
            return self;
        }

        self.segments.push(into);

        self
    }

    fn add_segment_p<T: Into<CowSegment<'a>>>(&mut self, prefix: &'a str, segment: T) -> &mut Self {
        self.add_segment(prefix).add_segment(segment)
    }

    fn add_segment_ps<T: Into<CowSegment<'a>>>(
        &mut self,
        prefix: &'a str,
        segment: T,
        suffix: &'a str,
    ) -> &mut Self {
        self.add_segment_p(prefix, segment).add_segment(suffix)
    }

    /// Add a parameter and its value to the query that will be used to replace all
    /// occurences of `key` into `value` when the `build` method is called.
    ///
    /// **IMPORTANT** Do not use this for user provided data, the input is not sanitized
    ///
    /// # Example
    /// ```
    /// use surreal_simple_querybuilder::prelude::*;
    ///
    /// let query = QueryBuilder::new()
    ///   .select("{{field}}")
    ///   .from("Account")
    ///   .param("{{field}}", "id")
    ///   .build();
    ///
    /// assert_eq!("SELECT id FROM Account", query);
    /// ```
    pub fn param(mut self, key: &'a str, value: &'a str) -> Self {
        self.parameters.insert(key, value);

        self
    }

    pub fn build(self) -> String {
        let mut output = self.segments.join(" ");

        for (key, value) in self.parameters {
            let key_size = key.len();

            while let Some(index) = output.find(key) {
                output.replace_range(index..index + key_size, value);
            }
        }

        output
    }
    /*
    /// Start a SET statement with all the public fields in the supplied `T` using
    /// the [SqlFieldSerializer] and Serde to list all the serializable fields in order
    /// to get a statement like the following:
    /// ```sql
    /// SET field_one = $field_one , field_two = $field_two
    /// ```
    ///
    /// The function is meant to be used with the models generated by the [model]
    /// macro.
    pub fn set_model<T: Serialize>(mut self, model: &T) -> SqlSerializeResult<Self> {
        let parameters = crate::model::to_parameters(model)?;

        self.add_segment_p("SET", parameters);

        Ok(self)
    }

    /// Start an UPDATE statement with all the public fields in the supplied `T` using
    /// the [SqlFieldSerializer] and Serde to list all the serializable fields in order
    /// to get a statement like the following:
    /// ```sql
    /// UPDATE field_one = $field_one , field_two = $field_two
    /// ```
    ///
    /// The function is meant to be used with the models generated by the [model]
    /// macro.
    pub fn update_model<T: Serialize>(mut self, model: &T) -> SqlSerializeResult<Self> {
        let parameters = crate::model::to_parameters(model)?;

        self.add_segment_p("UPDATE", parameters);

        Ok(self)
    } */
}
