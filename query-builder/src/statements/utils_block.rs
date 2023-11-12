/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{Buildable, Erroneous, Parametric, QueryChain, Queryable, ValueLike};

#[macro_export]
/// Macro for creating a surrealdb code block
/// # Examples
/// ```rust
/// // Use to create a return blocked that can be passed as a value in a statement.
///
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::*, functions::*};
/// # let alien = Table::new("alien");
/// # let metrics = Table::new("metrics");
/// # let strength = Field::new("strength");
///
/// let code_block_deprecated = block_deprecated! {
///     let strengths = select_value(strength).from(alien);
///     let total = math::sum!(strengths);
///     let count = count!(strengths);
///     return total.divide(count);
/// };
/// ```
/// ```rust, ignore
/// // Example passing as a value in a statement.
///     let created_stats_statement = create::<WeaponStats>(averageStrength.equal_to(block_deprecated! {
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
///     .query(block_deprecated!(
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
/// let transaction = block_deprecated! {
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
macro_rules! code_block_deprecated {
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

            use $crate::statements::utils::chain;


            let chain: $crate::statements::utils::QueryChain = $(
            chain(&$var).
            )*
            into();

            chain
        }
    };
  (IF ($condition:expr) THEN { $($then_body:expr;)+ } $(ELSE IF ($elif_condition:expr) THEN { $($elif_body:expr;)+ })* $(ELSE { $($else_body:expr;)+ })?) => {
        {
            let mut __statements: ::std::vec::Vec<$crate::statements::utils::Chainable> = ::std::vec::Vec::new();

            let if_condition = $crate::statements::if_($condition);
            let if_block = $crate::block_deprecated! { $($then_body;)+ };
            __statements.push(if_condition.then(if_block).into());

            $(
                let elif_condition = $crate::statements::if_($elif_condition);
                let elif_block = $crate::block_deprecated! { $($elif_body;)+ };
                __statements.push(elif_condition.else_if(elif_block).into());
            )*

            $(
                let else_block = $crate::block_deprecated! { $($else_body;)+ };
                __statements.push(else_block.into());
            )?

            $crate::statements::utils::QueryChain::from(__statements)
        }
    };
    (if ($condition:expr) then { $($then_body:expr;)+ } $(else if ($elif_condition:expr) then { $($elif_body:expr;)+ })* $(else { $($else_body:expr;)+ })?) => {
        {
            let mut __statements: ::std::vec::Vec<$crate::statements::utils::Chainable> = ::std::vec::Vec::new();

            let if_condition = $crate::statements::if_($condition);
            let if_block = $crate::block_deprecated! { $($then_body;)+ };
            __statements.push(if_condition.then(if_block).into());

            $(
                let elif_condition = $crate::statements::if_($elif_condition);
                let elif_block = $crate::block_deprecated! { $($elif_body;)+ };
                __statements.push(elif_condition.else_if(elif_block).into());
            )*

            $(
                let else_block = $crate::block_deprecated! { $($else_body;)+ };
                __statements.push(else_block.into());
            )?

            $crate::statements::utils::QueryChain::from(__statements)
        }
    };
    (FOR ($param:ident IN $iterable:expr) { $($stmt:expr;)+ }; $($rest:tt)*) => {
        {
            let ref $param = $crate::Param::new(stringify!($param));
            let for_loop = $crate::statements::for_($param).in_($iterable).block($crate::block_deprecated! { $($stmt;)+ });

            let mut __statements: ::std::vec::Vec<$crate::statements::utils::Chainable> = ::std::vec::Vec::new();
            __statements.push(for_loop.into());

            {
                $crate::block_deprecated_inner!( __statements; $($rest)*);
            }

            $crate::statements::utils::QueryChain::from(__statements)
        }
    };
    (for ($param:ident IN $iterable:expr) { $($stmt:expr;)+ }; $($rest:tt)*) => {
        {
            let ref $param = $crate::Param::new(stringify!($param));
            let for_loop = $crate::statements::for_($param).in_($iterable).block($crate::block_deprecated! { $($stmt;)+ });

            let mut __statements: ::std::vec::Vec<$crate::statements::utils::Chainable> = ::std::vec::Vec::new();
            __statements.push(for_loop.into());

            {
                $crate::block_deprecated_inner!( __statements; $($rest)*);
            }

            $crate::statements::utils::QueryChain::from(__statements)
        }
    };
    ($($query:expr;)*) => {
        {
            use $crate::statements::utils::chain;

            let chain: $crate::statements::utils::QueryChain = $(
            chain($query).
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

            use $crate::statements::utils::chain;

            let chain: $crate::statements::utils::QueryChain = $(
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

            use $crate::statements::utils::chain;

            let chain: $crate::statements::utils::QueryChain = $(
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

            use $crate::statements::utils::chain;

            let chain: $crate::statements::utils::QueryChain = $(
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

            use $crate::statements::utils::chain;

            let chain: $crate::statements::utils::QueryChain = $(
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

        use $crate::statements::utils::chain;

        let chain: $crate::statements::utils::QueryChain = $(
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

        use $crate::statements::utils::chain;

        let chain: $crate::statements::utils::QueryChain = $(
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
        let mut __statements: ::std::vec::Vec<$crate::statements::utils::Chainable> = ::std::vec::Vec::new();
        {
            $crate::block_deprecated_inner!( __statements; $($rest)*);
        }
        $crate::statements::utils::QueryChain::from(__statements)
    }};

    // () => {};
}
pub use code_block_deprecated as block_deprecated;

///  helper function for block macro
#[macro_export]
macro_rules! block_inner {
    ($statements:expr; let $var:ident = $value:expr; $($rest:tt)*) => {{
        let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
        $statements.push($var.clone().into());
        $crate::block_deprecated_inner!($statements; $($rest)*);
    }};
    ($statements:expr; LET $var:ident = $value:expr; $($rest:tt)*) => {{
        let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
        $statements.push($var.clone().into());
        $crate::block_deprecated_inner!($statements; $($rest)*);
    }};

    ($statements:expr; FOR ($param:ident IN $iterable:expr) { $($stmt:expr;)+ }; $($rest:tt)*) => {{
            let ref $param = $crate::Param::new(stringify!($param));
            let for_loop = $crate::statements::for_($param).in_($iterable).block($crate::block_deprecated! { $($stmt;)+ });

            $statements.push(for_loop.into());
            $crate::block_deprecated_inner!($statements; $($rest)*);
    }};

    ($statements:expr; for ($param:ident in $iterable:expr) { $($stmt:expr;)+ }; $($rest:tt)*) => {{
            let ref $param = $crate::Param::new(stringify!($param));
            let for_loop = $crate::statements::for_($param).in_($iterable).block($crate::block_deprecated! { $($stmt;)+ });

            $statements.push(for_loop.clone().into());
            $crate::block_deprecated_inner!($statements; $($rest)*);
    }};



    ($statements:expr; return $value:expr; $($rest:tt)*) => {{
        let __private_stmt = $crate::statements::return_($value);
        $statements.push(__private_stmt.into());
        $crate::block_deprecated_inner!($statements; $($rest)*);
    }};
    ($statements:expr; RETURN $value:expr; $($rest:tt)*) => {{
        let __private_stmt = $crate::statements::return_($value);
        $statements.push(__private_stmt.into());
        $crate::block_deprecated_inner!($statements; $($rest)*);
    }};
    ($statements:expr; $expr:expr; $($rest:tt)*) => {{
        $statements.push($expr.into());
        $crate::block_deprecated_inner!($statements; $($rest)*);
    }};
    ($statements:expr;) => {};
}

/// A code block. Surrounds the code with curly braces.
/// # Examples
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::*, functions::*};
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
/// let code_block_deprecated = block(chain(sales).chain(total).chain(count).chain(returned));
///
/// let def = define_field(average_sales).on_table(metrics).value(code_block_deprecated);
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

impl From<Block> for ValueLike {
    fn from(block: Block) -> Self {
        ValueLike {
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
