#![recursion_limit = "2048"]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use insta;
use regex;
use serde::{Deserialize, Serialize};
use static_assertions::*;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::Id,
    Result, Surreal,
};
use surrealdb_derive::{SurrealdbEdge, SurrealdbNode};

use std::fmt::{Debug, Display};
use surrealdb_macros::query_builder_old::query;
use surrealdb_macros::{
    links::{LinkMany, LinkOne, LinkSelf, Relate},
    model_id::SurId,
    Clause, SurrealdbEdge, SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,
    first_name: String,
    last_name: String,
    age: u8,

    #[surrealdb(link_self = "Student", skip_serializing)]
    best_friend: LinkSelf<Student>,

    #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "unoBook")]
    fav_book: LinkOne<Book>,

    #[surrealdb(link_one = "Book", skip_serializing)]
    course: LinkOne<Book>,

    #[surrealdb(link_many = "Book", skip_serializing)]
    #[serde(rename = "semCoures")]
    all_semester_courses: LinkMany<Book>,

    #[surrealdb(relate(model = "StudentWritesBook", connection = "->writes->book"))]
    written_books: Relate<Book>,
}

#[derive(SurrealdbEdge, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "writes")]
pub struct Writes<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,

    // #[surrealdb(link_one = "Book", skip_serializing)]
    #[serde(rename = "in")]
    _in: In,
    // r#in: In,
    out: Out,
    time_written: String,
}

type StudentWritesBook = Writes<Student, Book>;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "book")]
pub struct Book {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,
    title: String,
    content: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<SurId>,
    title: String,
    content: String,
}

trait WhiteSpaceRemoval {
    fn remove_extra_whitespace(&self) -> String
    where
        Self: std::borrow::Borrow<str>,
    {
        let mut result = String::with_capacity(self.borrow().len());
        let mut last_char_was_whitespace = true;

        for c in self.borrow().chars() {
            if c.is_whitespace() {
                if !last_char_was_whitespace {
                    result.push(' ');
                    last_char_was_whitespace = true;
                }
            } else {
                result.push(c);
                last_char_was_whitespace = false;
            }
        }

        result
    }
}
impl WhiteSpaceRemoval for &str {}
impl WhiteSpaceRemoval for String {}

// macro_rules! sql {
//     ($($item:tt)*) => {{
//         let valid_tokens = ["SELECT", "WHERE"];
//         let mut exprs = vec![];
//         $(
//             match stringify!($item) {
//                 $(
//                     x if x == valid_tokens[0] => {
//                         exprs.push(syn::parse_str(x).unwrap());
//                     }
//                 )*
//                 $(
//                     x if x == valid_tokens[1] => {
//                         exprs.push(syn::parse_str(x).unwrap());
//                     }
//                 )*
//                 _ => {
//                     exprs.push(syn::parse_str(stringify!($item)).unwrap_or_else(|_| {
//                         compile_error!(concat!("Invalid expression or token: ", stringify!($item)))
//                     }));
//                 }
//             }
//         )*
//         exprs
//     }};
// }

#[cfg(test)]
mod tests {
    #![recursion_limit = "256"]
    use super::*;
    // use surrealdb_macros::prelude::*;
    use surrealdb_macros::query_builder::{order, Order};
    use surrealdb_macros::{cond, query_builder};
    use surrealdb_macros::{q, DbField};
    use test_case::test_case;

    #[test]
    fn multiplication_tests1() {
        let student::Student {
            id,
            firstName,
            lastName,
            bestFriend,
            unoBook,
            course,
            semCoures,
            writtenBooks,
            age,
            ..
        } = &Student::schema();

        let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();
        let book::Book { content, .. } = Book::schema();

        let mut queryb = query_builder::QueryBuilder::new();

        let written_book_selection = Student::schema()
            .writes__(Clause::Where(timeWritten.equals("12:00")))
            .book(Clause::Where(content.contains("Oyelowo in Uranus")))
            .__as__(Student::schema().writtenBooks);

        #[derive(Serialize, Deserialize)]
        struct AND;

        impl Display for AND {
            fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
                f.write_str("AND")
            }
        }
        // age.and(firstName)

        let book::Book { content, .. } = Book::schema();

        let ref mut query1 = queryb
            .select_all()
            .from(Book::get_table_name())
            .where_(
                content.like("lowo").and(
                    age.greater_than_or_equal(600)
                        .or(firstName.equal("Oyelowo"))
                        .and(lastName.equal("Oyedayo")),
                ),
            )
            .group_by(content)
            .order_by(order(lastName).desc())
            .limit(50)
            .start(20)
            .timeout("15")
            .parallel();

        let is_lowo = true;
        if is_lowo {
            query1.limit(50);
            query1.group_by(age);
        }
        insta::assert_debug_snapshot!(query1.to_string());

        println!("XXXXXXXX {query1}");

        let ref mut query = queryb
            .select_all()
            .select(age)
            .select(firstName)
            .select_many(&[firstName, unoBook])
            .from(Student::get_table_name())
            .where_(
                age.greater_than_or_equals(18)
                    .or(firstName
                        .like("oyelowo")
                        .and(lastName.fuzzy_equal("oyedayo")))
                    .and(age.greater_than_or_equal(150)),
            )
            // .where_(cond!(age q!(>) "12:00" firstName q!(~) "lowo"))
            .order_by(order(firstName).rand().desc())
            .order_by(order(lastName).collate().asc())
            .order_by(order(id).numeric().desc())
            .order_by_many(&[order(id).numeric().desc(), order(firstName).desc()])
            .group_by(course)
            .group_by(firstName)
            .group_by(&"lastName".into())
            .group_by_many(&[lastName, unoBook, &DbField::new("lowo")])
            .start(5)
            .limit(400)
            .fetch(firstName)
            .fetch(lastName)
            .fetch_many(&[age, unoBook])
            .fetch_many(&[age, unoBook])
            .split(lastName)
            .split(firstName)
            .split_many(&[firstName, semCoures])
            .timeout("10s")
            .parallel();

        // let is_oyelowo = true;
        // if is_oyelowo {
        //     query.group_by_many(&[age, bestFriend, &DbField::new("lowo")]);
        // }
        //
        // stringify_tokens!("lowo", "knows", 5);

        // stringify_tokens2!("lowo", 5);
        let SELECT = "SELECT";
        let name = "name";
        let WHERE = "WHERE";
        let age = "age";

        // let result = sql!(SELECT name WHERE age > 5);
        // let result = sql!(SELECT name WHERE age > 5);

        // insta::assert_debug_snapshot!(query.to_string());

        // assert_eq!(
        //     query.to_string().remove_extra_whitespace(),
        //     "SELECT *, ->writes[WHERE timeWritten = 12:00]->book[WHERE \
        //     content CONTAINS Oyelowo in Uranus] AS writtenBooks FROM \
        //     WHERE age <= 12:00 GROUP BY course, firstName, lastName, \
        //     lastName, unoBook, lowo, age, bestFriend, lowo;"
        //         .remove_extra_whitespace()
        // )
    }

    #[test]
    fn multiplication_tests2() {
        let x = Student::schema()
            .writes__(Clause::All)
            .book(Clause::Id(SurId::try_from("book:blaze").unwrap()))
            .title;

        assert_eq!(
            x.to_string(),
            "->writes->book[WHERE id = book:blaze].title".to_string()
        )
    }

    #[test]
    fn multiplication_tests3() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::All)
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book.content".to_string()
        )
    }

    #[test]
    fn multiplication_tests4_with_alias() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(
                query().where_(Book::schema().content.contains("Oyelowo in Uranus")),
            ))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE content CONTAINS Oyelowo in Uranus] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests4() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Where(
                query().where_(Book::schema().content.contains("Oyelowo in Uranus")),
            ))
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE content CONTAINS Oyelowo in Uranus].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests5() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id(SurId::try_from("book:oyelowo").unwrap()))
            .content;

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo].content"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests6() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("book:oyelowo".try_into().unwrap()))
            .__as__(Student::schema().writtenBooks);

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo] AS writtenBooks"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests7() {
        let x = Student::schema()
            .writes__(Clause::Where(
                query().where_(StudentWritesBook::schema().timeWritten.equals("12:00")),
            ))
            .book(Clause::Id("book:oyelowo".try_into().unwrap()))
            .__as__("real_deal");

        assert_eq!(
            x.to_string(),
            "->writes[WHERE timeWritten = 12:00]->book[WHERE id = book:oyelowo] AS real_deal"
                .to_string()
        )
    }

    #[test]
    fn multiplication_tests8() {
        use serde_json;

        let sur_id = SurId::new("alien", "oyelowo");
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");

        let sur_id = SurId::try_from("alien:oyelowo").unwrap();
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");
    }
}
