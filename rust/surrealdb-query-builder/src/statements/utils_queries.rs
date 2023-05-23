// /// Write multiple queries in
// #[macro_export]
// macro_rules! surreal_queries {
//     ($($rest:tt)*) => {{
//         let mut __statements: ::std::vec::Vec<$crate::Chainable> = ::std::vec::Vec::new();
//         {
//             $crate::block_inner!( __statements; $($rest)*);
//         }
//         $crate::QueryChain::from(__statements)
//     }};
// }
//
// ///
// #[macro_export]
// macro_rules! block_inner {
//     ($statements:expr; let $var:ident = $value:expr; $($rest:tt)*) => {{
//         let ref $var = $crate::statements::let_(stringify!($var)).equal_to($value);
//         $statements.push($var.clone().into());
//         $crate::block_inner!($statements; $($rest)*);
//     }};
//     ($statements:expr; return $value:expr; $($rest:tt)*) => {{
//         let __stmt = $crate::statements::return_($value);
//         $statements.push(__stmt.into());
//         $crate::block_inner!($statements; $($rest)*);
//     }};
//     ($statements:expr; $expr:expr; $($rest:tt)*) => {{
//         $statements.push($expr.into());
//         $crate::block_inner!($statements; $($rest)*);
//     }};
//     ($statements:expr;) => {};
// }
//
// pub use surreal_queries as queries;
