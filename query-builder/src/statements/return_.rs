/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{BindingsList, Buildable, Erroneous, ErrorList, Parametric, Queryable, ValueLike};

use super::select::Fetchables;

/// The `RETURN` statement.
///
/// # Arguemnt
/// * `return_value` - The value to return.
///
/// # Examples
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::*, functions::*};
///
/// let user = TestUser::create_id("oyelowo");
/// let account = Field::new("account");
/// let connection = Field::new("connection");
///
/// let query = return_(user).fetch(&[account, connection]);
/// assert_eq!(
///     query.to_raw().build(),
///     "RETURN user:oyelowo FETCH account, connection;"
/// );
/// ```
pub fn return_(return_value: impl Into<ValueLike>) -> ReturnStatement {
    let return_value: ValueLike = return_value.into();
    let return_string = return_value.build();
    let bindings = return_value.get_bindings();
    let errors = return_value.get_errors();

    ReturnStatement {
        return_value: return_string,
        fetch: vec![],
        bindings,
        errors,
    }
}

/// The `RETURN` statement.
pub struct ReturnStatement {
    return_value: String,
    fetch: Vec<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl ReturnStatement {
    /// Fetches the specified fields.
    pub fn fetch(mut self, fetchables: impl Into<Fetchables>) -> Self {
        let fields: Fetchables = fetchables.into();

        let fields = match fields {
            Fetchables::Field(one_field) => vec![one_field],
            Fetchables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.fetch.push(f.build());
            self.bindings.extend(f.get_bindings());
            self.errors.extend(f.get_errors());
        });
        self
    }
}

impl Buildable for ReturnStatement {
    fn build(&self) -> String {
        let mut query = format!("RETURN {}", self.return_value);
        if !self.fetch.is_empty() {
            query = format!("{query} FETCH {}", self.fetch.join(", "));
        }

        format!("{query};")
    }
}

impl Parametric for ReturnStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Erroneous for ReturnStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.clone()
    }
}

impl Queryable for ReturnStatement {}

#[cfg(test)]
mod tests {
    use select::select_value;

    use crate::{
        block, bracket, chain,
        functions::{count, math},
        statements::{define_field, let_, select},
        Field, Model, Operatable, Table, TestUser, ToRaw,
    };

    use super::*;

    #[test]
    fn test_return() {
        let query = return_(1);
        assert_eq!(query.fine_tune_params(), "RETURN $_param_00000001;");
        assert_eq!(query.to_raw().build(), "RETURN 1;");
    }

    #[test]
    fn test_return_with_record() {
        let user = TestUser::create_id("oyelowo");
        let account = Field::new("account");
        let connection = Field::new("connection");

        let query = return_(user).fetch(&[account, connection]);
        assert_eq!(
            query.fine_tune_params(),
            "RETURN $_param_00000001 FETCH account, connection;"
        );
        assert_eq!(
            query.to_raw().build(),
            "RETURN user:oyelowo FETCH account, connection;"
        );
    }

    #[test]
    fn test_return_fetch() {
        let query = return_(1).fetch("a");
        assert_eq!(query.fine_tune_params(), "RETURN $_param_00000001 FETCH a;");
        assert_eq!(query.to_raw().build(), "RETURN 1 FETCH a;");
    }

    #[test]
    fn test_return_fetches() {
        let name = Field::new("name");
        let age = Field::new("age");
        let query = return_(1).fetch(vec![name, age]);
        assert_eq!(
            query.fine_tune_params(),
            "RETURN $_param_00000001 FETCH name, age;"
        );
        assert_eq!(query.to_raw().build(), "RETURN 1 FETCH name, age;");
    }

    #[test]
    fn test_return_with_math() {
        let sales = Table::new("sales");
        let metrics = Table::new("metrics");
        let quantity = Field::new("quantity");
        let average_sales = Field::new("average_sales");

        let sales = &let_("sales").equal_to(select_value(quantity).from(sales));
        let total = &let_("total").equal_to(math::sum!(sales));
        let count = &let_("count").equal_to(count!(sales));

        let returned = return_(bracket(total.divide(count)));

        let def = define_field(average_sales).on_table(metrics).value(block(
            chain(sales).chain(total).chain(count).chain(returned),
        ));

        insta::assert_snapshot!(def.to_raw());

        insta::assert_snapshot!(def.fine_tune_params());

        assert_eq!(
            def.to_raw().build(),
            "DEFINE FIELD average_sales ON TABLE metrics VALUE $value OR {\n\
                LET $sales = (SELECT VALUE quantity FROM sales);\n\n\
                LET $total = math::sum($sales);\n\n\
                LET $count = count($sales);\n\n\
                RETURN ($total / $count);\n\
                };"
        );
    }
}
