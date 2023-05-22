/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{statements::QueryChain, Buildable, Erroneous, Parametric, Queryable, Valuex};

#[macro_export]
/// Macro for creating a surrealdb code block
/// # Examples
/// ```
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::*, functions::*};
/// # let alien = Table::new("alien");
/// # let metrics = Table::new("metrics");
/// # let strength = Field::new("strength");
///
/// let code_block = block! {
///     let strengths = select_value(strength).from(alien);
///     let total = math::sum!(&strengths);
///     let count = count!(&strengths);
///     return total.divide(count);
/// };
/// ```
macro_rules! code_block {
    ($(let $var:ident = $value:expr;)* return $expr:expr;) => {
        {
            $(
                let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            $crate::statements::
            $(
                chain($var).
            )*

            chain($crate::statements::return_($expr)).as_block()
        }
    };
    ($(let $var:ident = $value:expr;)*) => {
        {
            $(
                let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            use $crate::statements::chain;


            let chain: $crate::statements::QueryChain = $(
            chain(&$var).
            )*
            into();

            chain
        }
    };
    ($($query:expr;)*) => {
        {
            use $crate::statements::chain;

            let chain: $crate::statements::QueryChain = $(
            chain(&$query).
            )*
            into();

            chain
        }
    };
    (BEGIN TRANSACTION; $(let $var:ident = $value:expr;)* COMMIT TRANSACTION;) => {
        {
            $(
                let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            use $crate::statements::chain;

            let chain: $crate::statements::QueryChain = $(
                chain(&$var).
                )*
                into();

            $crate::statements::begin_transaction().
            query(chain)
            .commit_transaction()
        }
    };
    (BEGIN TRANSACTION; $(let $var:ident = $value:expr;)* CANCEL TRANSACTION;) => {
        {
            $(
                let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            use $crate::statements::chain;

            let chain: $crate::statements::QueryChain = $(
                chain(&$var).
                )*
                into();

            $crate::statements::begin_transaction().
            query(chain)
            .cancel_transaction()
        }
    };
    (BEGIN TRANSACTION; $($query:expr;)* COMMIT TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (begin transaction; $($query:expr;)* commit transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (begin transaction;$(let $var:ident = $value:expr;)* commit transaction;) => {
        $(
            let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
        )*

        use $crate::statements::chain;

        let chain: $crate::statements::QueryChain = $(
            chain(&$var).
            )*
            into();

        $crate::statements::begin_transaction().
        $(
        query(&chain).
        )*

        .commit_transaction()
    };
    (BEGIN TRANSACTION; $($query:expr;)* CANCEL TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
    (begin transaction; $($query:expr;)* cancel transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
    ($statement:stmt) => {{
        $statement
    }};
    // () => {};
}
pub use code_block as block;

/// Write multiple queries in
#[macro_export]
macro_rules! surreal_queries {
    ($($rest:tt)*) => {{
        let mut __statements: Vec<$crate::statements::Chainable> = Vec::new();
        {
            $crate::block_inner!( __statements; $($rest)*);
        }
        // $crate::Block::from($crate::statements::QueryChain::from(__statements))
        $crate::statements::QueryChain::from(__statements)
    }};
}

///
#[macro_export]
macro_rules! block_inner {
    ($statements:expr; let $var:ident = $value:expr; $($rest:tt)*) => {{
        let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
        $statements.push($var.clone().into());
        $crate::block_inner!($statements; $($rest)*);
    }};
    ($statements:expr; return $value:expr; $($rest:tt)*) => {{
        let __stmt = $crate::statements::return_($value);
        $statements.push(__stmt.into());
        $crate::block_inner!($statements; $($rest)*);
    }};
    ($statements:expr; $expr:expr; $($rest:tt)*) => {{
        $statements.push($expr.into());
        $crate::block_inner!($statements; $($rest)*);
    }};
    ($statements:expr;) => {};
}

pub use surreal_queries as queries;

/// A code block. Surrounds the code with curly braces.
/// # Examples
/// ```
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::*, functions::*};
///
/// let sales = Table::new("sales");
/// let metrics = Table::new("metrics");
/// let quantity = Field::new("quantity");
/// let average_sales = Field::new("average_sales");
///
/// let ref sales = let_("sales").equal_to(select_value(quantity).from(sales));
/// let ref total = let_("total").equal_to(math::sum!(sales));
/// let ref count = let_("count").equal_to(count!(sales));
///
/// let returned = return_(bracket(total.divide(count)));
/// let code_block = block(chain(sales).chain(total).chain(count).chain(returned));
///
/// let def = define_field(average_sales).on_table(metrics).value(code_block);
///
/// assert_eq!(
///     def.to_raw().build(),
///     "DEFINE FIELD average_sales ON TABLE metrics VALUE $value OR {\n\
///         LET $sales = (SELECT VALUE quantity FROM sales);\n\n\
///         LET $total = math::sum($sales);\n\n\
///         LET $count = count($sales);\n\n\
///         RETURN ($total / $count);\n\
///         };"
/// );
pub fn block(code: QueryChain) -> Block {
    Block(code)
}

/// A code block. Surrounds the code with curly braces.
#[derive(Debug, Clone)]
pub struct Block(QueryChain);

impl Buildable for Block {
    fn build(&self) -> String {
        // format!("{};", self.0.build().trim_end_matches(";"))
        format!("{{\n{}\n}}", self.0.build())
    }
}

impl From<QueryChain> for Block {
    fn from(code: QueryChain) -> Self {
        Self(code)
    }
}

impl From<Block> for Valuex {
    fn from(block: Block) -> Self {
        Valuex {
            string: block.build(),
            bindings: block.get_bindings(),
            errors: block.get_errors(),
        }
    }
}

impl Queryable for Block {}

impl Parametric for Block {
    fn get_bindings(&self) -> crate::BindingsList {
        self.0.get_bindings()
    }
}

impl Erroneous for Block {
    fn get_errors(&self) -> crate::ErrorList {
        self.0.get_errors()
    }
}
