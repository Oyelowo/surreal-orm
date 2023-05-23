/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{Buildable, Erroneous, Parametric, QueryChain, Queryable, Valuex};

#[macro_export]
/// Macro for creating a surrealdb code block
/// # Examples
/// ```rust
/// // Use to create a return blocked that can be passed as a value in a statement.
///
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::*, functions::*};
/// # let alien = Table::new("alien");
/// # let metrics = Table::new("metrics");
/// # let strength = Field::new("strength");
///
/// let code_block = block! {
///     let strengths = select_value(strength).from(alien);
///     let total = math::sum!(strengths);
///     let count = count!(strengths);
///     return total.divide(count);
/// };
/// ```
/// ```rust, ignore
/// // Example passing as a value in a statement.
///     let created_stats_statement = create::<WeaponStats>(averageStrength.equal_to(block! {
///     let strengths = select_value(strength).from(weapon);
///     let total = math::sum!(strengths);
///     let count = count!(strengths);
///     return total.divide(count);
/// }));
/// ```
/// ```rust, ignore
/// // Using in a transaction query function.
///
/// let id1 = Account::create_id("one".into());
/// let id2 = Account::create_id("two".into());
/// let acc = Account::schema();
///
/// let amount_to_transfer = 300.00;
/// let transaction_query = begin_transaction()
///     .query(block!(
///         let balance = create(Balance {
///             id: Balance::create_id("balance".into()),
///             balance: amount_to_transfer,
///         });
///
///         create(Account {
///             id: id1,
///             balance: 135_605.16,
///         });
///
///         create(Account {
///             id: id2,
///             balance: 91_031.31,
///         });
///
///         update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).balance));
///         update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
///     ))
///     .commit_transaction();
///
/// transaction_query.run(db.clone()).await?;
/// ```
/// ```rust, ignore
/// // Using to create a transaction query function.
/// let id1 = Account::create_id("one".into());
/// let id2 = Account::create_id("two".into());
/// let amount_to_transfer = 300.00;
/// let acc = Account::schema();
///
/// let transaction = block! {
///     BEGIN TRANSACTION;
///
///     let acc1 = create(Account {
///         id: id1,
///         balance: 135_605.16,
///     });
///     let acc2 = create(Account {
///         id: id2,
///         balance: 91_031.31,
///     });
///
///     let updated1 = update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer));
///     let update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
///
///     COMMIT TRANSACTION;
/// };
/// ```
macro_rules! code_block {
    ($(let $var:ident = $value:expr;)* return $expr:expr;) => {
        {
            $(
                let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            $crate::
            $(
                chain($var).
            )*

            chain($crate::statements::return_($expr)).as_block()
        }
    };
    ($(LET $var:ident = $value:expr;)* RETURN $expr:expr;) => {
        {
            $(
                let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            $crate::
            $(
                chain($var).
            )*

            chain($crate::statements::return_($expr)).as_block()
        }
    };
    ($(LET $var:ident = $value:expr;)*) => {
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
    (BEGIN TRANSACTION; $(LET $var:ident = $value:expr;)* COMMIT TRANSACTION;) => {
        {
            $(
                let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            use $crate::chain;

            let chain: $crate::QueryChain = $(
                chain(&$var).
                )*
                into();

            $crate::statements::begin_transaction().
            query(chain)
            .commit_transaction()
        }
    };
    (BEGIN TRANSACTION; $(let $var:ident = $value:expr;)* COMMIT TRANSACTION;) => {
        {
            $(
                let $var = $crate::statements::let_(stringify!($var)).equal_to($value);
            )*

            use $crate::chain;

            let chain: $crate::QueryChain = $(
                chain(&$var).
                )*
                into();

            $crate::statements::begin_transaction().
            query(chain)
            .commit_transaction()
        }
    };
    (BEGIN TRANSACTION; $(LET $var:ident = $value:expr;)* CANCEL TRANSACTION;) => {
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
    (begin transaction;$(LET $var:ident = $value:expr;)* commit transaction;) => {
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
    ($($rest:tt)*) => {{
        let mut __statements: ::std::vec::Vec<$crate::Chainable> = ::std::vec::Vec::new();
        {
            $crate::block_inner!( __statements; $($rest)*);
        }
        $crate::QueryChain::from(__statements)
    }};

    // () => {};
}
pub use code_block as block;

///  helper function for block macro
#[macro_export]
macro_rules! block_inner {
    ($statements:expr; let $var:ident = $value:expr; $($rest:tt)*) => {{
        let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
        $statements.push($var.clone().into());
        $crate::block_inner!($statements; $($rest)*);
    }};
    ($statements:expr; LET $var:ident = $value:expr; $($rest:tt)*) => {{
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
