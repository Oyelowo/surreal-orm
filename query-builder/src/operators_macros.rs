/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

/// This macro is used to create the different operators that can be used in the query builder.
#[macro_export]
macro_rules! q {
    (=) => {
        "="
    };
    (!=) => {
        "!="
    };
    (==) => {
        "=="
    };
    (?=) => {
        "?="
    };
    (*=) => {
        "*="
    };
    (~) => {
        "~"
    };
    (!~) => {
        "!~"
    };
    (?~) => {
        "?~"
    };
    (*~) => {
        "*~"
    };
    (<) => {
        "<"
    };
    (<=) => {
        "<="
    };
    (>) => {
        ">"
    };
    (>=) => {
        ">="
    };
    (+) => {
        "+"
    };
    (-) => {
        "-"
    };
    (*) => {
        "*"
    };
    (/) => {
        "/"
    };
    (&&) => {
        "&&"
    };
    (||) => {
        "||"
    };
    (AND) => {
        "AND"
    };
    (OR) => {
        "OR"
    };
    (IS) => {
        "IS"
    };
    (IS NOT) => {
        "IS NOT"
    };
    (CONTAINS) => {
        "CONTAINS"
    };
    ("∋") => {
        "∋"
    };
    (CONTAINSNOT) => {
        "CONTAINSNOT"
    };
    ("∌") => {
        "∌"
    };
    (CONTAINSALL) => {
        "CONTAINSALL"
    };
    ("⊇") => {
        "⊇"
    };
    (CONTAINSANY) => {
        "CONTAINSANY"
    };
    ("⊃") => {
        "⊃"
    };
    (CONTAINSNONE) => {
        "CONTAINSNONE"
    };
    ("⊅") => {
        "⊅"
    };
    (INSIDE) => {
        "INSIDE"
    };
    ("∈") => {
        "∈"
    };
    (NOTINSIDE) => {
        "NOTINSIDE"
    };
    ("∉") => {
        "∉"
    };
    (ALLINSIDE) => {
        "ALLINSIDE"
    };
    ("⊆") => {
        "⊆"
    };
    (ANYINSIDE) => {
        "ANYINSIDE"
    };
    ("⊂") => {
        "⊂"
    };
    (NONEINSIDE) => {
        "NONEINSIDE"
    };
    ("⊄") => {
        "⊄"
    };
    (OUTSIDE) => {
        "OUTSIDE"
    };
    (INTERSECTS) => {
        "INTERSECTS"
    };
    ($other: expr) => {
        compile_error!(concat!("Invalid operator!: ", $other));
    };
}

// #[macro_export]
// macro_rules! cond {
//     ($($expr:expr)*) => {
//         {
//             let mut v = Vec::new();
//             $(
//                 v.push($expr.to_string());
//             )*
//             v.join(" ")
//         }
//     };
// }

// macro_rules! wher_ {
//     ($left: expr op!($op: tt) $right: expr) => {
//         [$left.to_string().as_str(), stringify!($op), $right]
//     };
// }
