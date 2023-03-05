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
use surrealdb_macros::{
    links::{LinkMany, LinkOne, LinkSelf, Relate},
    RecordId, SurrealdbEdge, SurrealdbNode,
};
use test_case::test_case;
use typed_builder::TypedBuilder;

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "student")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
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
    id: Option<RecordId>,

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
    id: Option<RecordId>,
    title: String,
    content: String,
}

#[derive(SurrealdbNode, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "blog")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<RecordId>,
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
    use super::*;
    use _core::time::Duration;
    use surrealdb::sql;
    use surrealdb_macros::db_field::{cond, empty, Binding, Empty, Parametric};
    // use surrealdb_macros::prelude::*;
    use surrealdb_macros::query_select::{order, select, All, Order};
    use surrealdb_macros::value_type_wrappers::SurrealId;
    use surrealdb_macros::{cond, query_select, DbFilter};
    use surrealdb_macros::{q, DbField};
    use test_case::test_case;

    fn mana(v: impl Into<DbFilter>) {
        let x: DbFilter = v.into();

        let m = x.bracketed();
        println!("OFEMR>>>>{m}");
        // let xx = DbFilter::from(v);
    }

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
        let st = Student::schema();
        let bk = &Book::schema();
        let wrt = &StudentWritesBook::schema();
        let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();
        let book::Book { content, .. } = Book::schema();

        mana(bk.content.contains("Lowo"));
        mana(None);

        fn where___(xx: impl Into<DbFilter>) {}
        where___(firstName);
        where___(
            cond(
                age.less_than_or_equal(18)
                    .greater_than_or_equal(age)
                    .add(age)
                    .subtract(18)
                    .divide(firstName)
                    .multiply(lastName)
                    .greater_than_or_equal(course),
            )
            .or(lastName.greater_than_or_equal(4)),
        );

        where___(firstName.like("oyelowo"));

        let x = firstName.like("oeere");
        where___(x);

        let x = cond(firstName.like("oeere"));
        where___(x);
        where___(None);

        where___(
            cond(age.add(1).multiply(2).equal(course.divide(2).subtract(1)))
                // TODO: Make it possible to do &[1, 2] or vec![1, 2] without explicitly specifying the
                // integer type
                .and(age.all_inside(vec![1, 2]))
                .and(age.all_inside(&[1, 2]))
                .and(cond(firstName.like("D")).and(lastName.like("E"))),
        );
        where___(
            cond(firstName.like("oyelowo"))
                .and(lastName.like("oyedayo"))
                .or(age
                    .greater_than_or_equal(age)
                    .greater_than_or_equal(age)
                    .add(age)
                    .add(age)
                    .subtract(age)
                    .divide(age)
                    .multiply(age)
                    .or(firstName)
                    .intersects(age))
                .and(lastName.greater_than_or_equal(45).or(course))
                .and(
                    firstName
                        .any_equal(vec!["asrer"])
                        .greater_than_or_equal(50)
                        .subtract(5)
                        .less_than_or_equal(200),
                ),
        );

        // Chain::new(age.clone())
        //     .chain(firstName)
        //     .greater_than_or_equal(20);

        // let xx = firstName
        //     .less_than(age)
        //     .greater_than(age)
        //     .less_than(firstName)
        //     .add(age)
        //     .multiply(id)
        //     .subtract(bestFriend)
        //     .divide(course)
        //     .fuzzy_equal(unoBook)
        //     .and(firstName.greater_than(age))
        //     .or(age.less_than_or_equal(writtenBooks))
        //     .or(age.greater_than(age));
        //
        // println!("maerfineirNAMAAAA :{xx}");

        let written_book_selection = st
            .bestFriend(Empty)
            .writes__(wrt.timeWritten.equal("12:00"))
            .book(bk.content.contains("Oyelowo in Uranus"))
            .__as__(st.writtenBooks);

        let st = Student::schema();
        let written_book_selection = st
            .bestFriend(Empty)
            .writes__(wrt.timeWritten.equal("12:00"))
            .book(bk.content.contains("Oyelowo in Uranus"))
            .__as__(st.writtenBooks);

        let rer = "".to_string().is_empty();

        #[derive(Serialize, Deserialize)]
        struct LIKE;

        impl Display for LIKE {
            fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
                f.write_str("LIKE")
            }
        }
        #[derive(Serialize, Deserialize)]
        struct OR;

        impl Display for OR {
            fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
                f.write_str("OR")
            }
        }
        #[derive(Serialize, Deserialize)]
        struct AND;

        impl Display for AND {
            fn fmt(&self, f: &mut _core::fmt::Formatter<'_>) -> _core::fmt::Result {
                f.write_str("AND")
            }
        }
        // age.and(firstName)

        let book::Book { content, .. } = Book::schema();

        content
            .contains_any(vec!["Dyayo", "fdfd"])
            .contains_any(&["Dyayo", "fdfd"])
            .contains_all(vec!["Dyayo", "fdfd"])
            .contains_all(&["Dyayo", "fdfd"])
            .contains_none(vec!["Dyayo", "fdfd"])
            .contains_none(&["Dyayo", "fdfd"])
            .contains_none(vec![1, 3])
            .contains_none(&[1, 3])
            .or("lowo")
            .and(age.less_than(55))
            .or(age.greater_than(17))
            .or(firstName.equal("Oyelowo"))
            .and(lastName.equal("Oyedayo"));

        let mut query1 = select::<Book>(All)
            .from(Book::get_table_name())
            .where_(
                cond(content.like("lowo").and(age).greater_than_or_equal(600))
                    .or(firstName.equal("Oyelowo"))
                    .and(lastName.equal("Oyedayo")),
            )
            .group_by(content)
            .order_by(order(lastName).desc())
            .limit(50)
            .start(20)
            .timeout(Duration::from_secs(9))
            .parallel();

        let is_lowo = true;
        if is_lowo {
            query1 = query1.limit(50).group_by(age);
        }

        insta::assert_debug_snapshot!(query1.to_string());
        insta::assert_debug_snapshot!(query1.get_bindings());

        let ref student_table = Student::get_table_name();
        let ref book_table = Book::get_table_name();
        let ref book_id = SurrealId::try_from("book:1").unwrap();
        let ref student_id = SurrealId::try_from("student:1").unwrap();

        let mut query = select(All)
            .select(age)
            .select(firstName)
            .select(&[firstName, unoBook])
            .select(vec![firstName, unoBook])
            .from(student_table)
            .from(&[student_table, book_table])
            .from(vec![student_table, book_table])
            .from(book_id)
            .from(&[book_id, student_id])
            .from(vec![book_id, student_id])
            .from(vec![SurrealId::try_from("book:1").unwrap()])
            .from(query1)
            .where_(
                cond(
                    age.greater_than(age)
                        .greater_than_or_equal(age)
                        .less_than_or_equal(20)
                        .like(firstName)
                        .add(5)
                        .subtract(10)
                        .and(unoBook)
                        .or(age),
                )
                .and(bestFriend.exactly_equal("Oyelowo"))
                .or(firstName.equal(true))
                .and(age.greater_than_or_equal(150)),
            )
            // .where_(
            //     cond!(age q!(>=) "12:00" OR firstName LIKE "oyelowo" AND lastName q!(~) "oyedyao"  AND age q!(>) 150),
            // )
            .order_by(order(firstName).rand().desc())
            .order_by(order(lastName).collate().asc())
            .order_by(order(id).numeric().desc())
            .order_by(vec![order(id).numeric().desc()])
            .order_by(&[order(id).numeric().desc(), order(firstName).desc()])
            .order_by(&[order(id).numeric().desc(), order(firstName).desc()])
            .group_by(course)
            .group_by(firstName)
            .group_by(&[lastName, unoBook, &DbField::new("lowo")])
            .group_by(vec![lastName, unoBook, &DbField::new("lowo")])
            .start(5)
            .limit(400)
            .fetch(firstName)
            .fetch(lastName)
            .fetch(&[age, unoBook])
            .fetch(vec![age, unoBook])
            .split(lastName)
            .split(firstName)
            .split(&[firstName, semCoures])
            .split(vec![firstName, semCoures])
            .timeout(Duration::from_secs(8))
            .parallel();

        let is_oyelowo = true;
        if is_oyelowo {
            query = query.group_by(&[age, bestFriend, &DbField::new("dayo")]);
        }

        // stringify_tokens!("lowo", "knows", 5);

        // stringify_tokens2!("lowo", 5);
        let SELECT = "SELECT";
        let name = "name";
        let WHERE = "WHERE";
        let age = "age";

        // let result = sql!(SELECT name WHERE age > 5);
        // let result = sql!(SELECT name WHERE age > 5);

        insta::assert_debug_snapshot!(query.to_string());
        insta::assert_debug_snapshot!(query.get_bindings());

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
            .writes__(Empty)
            .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))))
            .title;

        assert_eq!(
            x.to_string(),
            // "->writes->book[WHERE id = book:blaze].title".to_string()
            "->writes->book[WHERE id = $_param_00000000].title".to_string()
        );

        let m = x.get_bindings();
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            "[[\"_param_00000000\",\"book:blaze\"]]".to_string()
        );

        let student = Student::schema();
        // Another case
        let x = student
            .bestFriend(student.age.between(18, 150))
            .bestFriend(Empty)
            .writes__(StudentWritesBook::schema().timeWritten.greater_than(3422))
            .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))));

        insta::assert_display_snapshot!(x);
        insta::assert_debug_snapshot!(x.get_bindings());
        // assert_eq!(
        //     x.to_string(),
        //     // "->writes->book[WHERE id = book:blaze].title".to_string()
        //     "->writes->book[WHERE id = $_param_00000000].title".to_string()
        // );
        //
        // let m = x.get_bindings();
        // assert_eq!(
        //     serde_json::to_string(&m).unwrap(),
        //     "[[\"_param_00000000\",\"book:blaze\"]]".to_string()
        // );
        // assert_eq!(
        //     format!("{m:?}"),
        //     "[[\"_param_00000000\",\"book:blaze\"]]".to_string()
        // );
        // let query = InsertQuery::new("company")
        //     .fields(&["name", "founded", "founders", "tags"])
        //     .values(&[
        //         &[
        //             "SurrealDB",
        //             "2021-09-10",
        //             "[person:tobie, person:jaime]",
        //             "['big data', 'database']",
        //         ],
        //         &["Acme Inc.", "1967-05-03", "null", "null"],
        //         &["Apple Inc.", "1976-04-01", "null", "null"],
        //     ])
        //     .on_duplicate_key_update(&[("tags", "tags += 'new tag'")])
        //     .build();
        // println!("{}", query);
    }

    #[test]
    fn multiplication_tests3() {
        let x = Student::schema()
            .writes__(StudentWritesBook::schema().timeWritten.equal("12:00"))
            .book(empty())
            .content;

        assert_eq!(
            x.to_string(),
            // "->writes[WHERE timeWritten = 12:00]->book.content".to_string()
            "->writes[WHERE timeWritten = $_param_00000000]->book.content".to_string()
        )
    }

    #[test]
    fn multiplication_tests8() {
        use serde_json;

        let sur_id = SurrealId::try_from("alien:oyelowo").unwrap();
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");

        let sur_id = RecordId::from(("alien", "oyelowo"));
        let json = serde_json::to_string(&sur_id).unwrap();
        assert_eq!(json, "\"alien:oyelowo\"");
    }
}
