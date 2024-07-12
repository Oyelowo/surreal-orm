/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use std::time::Duration;
use surreal_models::{book, student, Book, Student, StudentWritesBook};
use surreal_orm::{
    cond,
    statements::{order, relate, select},
    *,
};
use surrealdb::sql::{thing, Thing};

#[test]
fn multiplication_tests1() {
    let student::Schema {
        id,
        firstName,
        lastName,
        bestFriend,
        unoBook,
        course,
        semesterCourses,
        age,
        ..
    } = &Student::schema();
    let book::Schema { content, .. } = &Book::schema();
    let duration = Duration::from_secs(9);

    let mut query1 = select(arr![age, lastName, content])
        .from(Book::table())
        .where_(
            cond(content.like("lowo"))
                .and(age.gte(600))
                .or(firstName.eq("Jupiter"))
                .and(lastName.eq("Pluto")),
        )
        .order_by(lastName.desc())
        .limit(50)
        .start(20)
        .timeout(duration)
        .parallel();

    let is_true = true;
    if is_true {
        query1 = query1.limit(55).order_by(order(age).desc());
    }

    insta::assert_snapshot!(&query1.fine_tune_params());

    let student_table = &Student::get_table();
    let _book_table = &Book::get_table();
    let _book_id = &thing("book:1").unwrap();

    let mut query = select(arr![All, content, age, lastName, firstName, course])
        // Also work
        // .select(age)
        // .select(firstName)
        // .select(&[firstName, unoBook])
        // .select(vec![firstName, unoBook])
        .from(student_table)
        // .from(&[student_table, book_table])
        // .from(vec![student_table, book_table])
        // .from(book_id)
        // .from(&[book_id, student_id])
        // .from(vec![book_id, student_id])
        // .from(vec![thing("book:1")])
        // .from(query1)
        .where_(
            cond(
                age.greater_than(age)
                    .like(firstName)
                    .add(5)
                    .subtract(10)
                    .and(unoBook)
                    .or(age),
            )
            .and(bestFriend.exactly_equal("Oyelowo"))
            .or(firstName.equal_to("Oyedayo"))
            .and(age.greater_than_or_equal(150)),
        )
        // .where_(
        //     cond!(age q!(>=) "12:00" OR firstName LIKE "oyelowo" AND lastName q!(~) "oyedyao"  AND age q!(>) 150),
        // )
        .order_by(firstName.rand().desc())
        .order_by(lastName.collate().asc())
        .order_by(id.numeric().desc())
        .order_by(vec![id.numeric().desc()])
        .order_by(&[id.numeric().desc(), firstName.desc()])
        .order_by(&[id.numeric().desc(), firstName.desc()])
        .group_by(course)
        .group_by(firstName)
        .group_by(arr![lastName, unoBook])
        .group_by(arr![lastName, unoBook])
        // .group_by(&[lastName, unoBook, &Field::new("lowo")])
        // .group_by(vec![lastName, unoBook, &Field::new("lowo")])
        .start(5)
        .limit(400)
        .fetch(firstName)
        .fetch(lastName)
        .fetch(arr![age, unoBook])
        .fetch(arr![age, unoBook])
        // .fetch(&[age, unoBook])
        // .fetch(vec![age, unoBook])
        .split(lastName)
        .split(firstName)
        .split(arr![firstName, semesterCourses])
        .split(arr![firstName, semesterCourses])
        // .split(&[firstName, semesterCourses])
        // .split(vec![firstName, semesterCourses])
        .timeout(Duration::from_secs(8))
        .parallel();

    let is_oyelowo = true;
    if is_oyelowo {
        query = query.group_by(arr![age, bestFriend, &Field::new("dayo")]);
    }

    insta::assert_snapshot!(&query.fine_tune_params());
    insta::assert_snapshot!(&query.to_raw().build());
    insta::assert_snapshot!(&query.get_bindings().len());
}

#[tokio::test]
async fn relate_query_building_for_ids() {
    let student_id = &thing("student:1").unwrap();
    let book_id = &thing("book:2").unwrap();
    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    let relate_simple =
        relate(Student::with(student_id).writes__(Empty).book(book_id)).content(write);

    insta::assert_snapshot!(&relate_simple.fine_tune_params());
    assert_eq!(relate_simple.clone().to_raw().build().len(), 147);
    assert_eq!(relate_simple.clone().get_bindings().len(), 3);
}

#[tokio::test]
async fn relate_query_building_for_subqueries() {
    let write = StudentWritesBook {
        time_written: Duration::from_secs(52),
        ..Default::default()
    };
    let relation = relate(
        Student::with(select(All).from(Student::get_table()))
            .writes__(Empty)
            .book(
                select(All)
                    .from(Book::get_table())
                    .where_(Book::schema().title.like("Oyelowo")),
            ),
    )
    .content(write);
    insta::assert_snapshot!(relation.fine_tune_params());
    assert_eq!(relation.get_bindings().len(), 3);
}

#[test]
fn multiplication_tests2() {
    let _simple_relation = Student::schema()
        .writes__(Empty)
        .book(Book::schema().id.equal(Thing::from(("book", "blaze"))))
        .title;

    let st_schema = &Student::schema();
    // Another case
    let cursive_relation = st_schema
        .bestFriend()
        .bestFriend()
        .writes__(StudentWritesBook::schema().timeWritten.greater_than(3422))
        .book(Book::schema().id.equal(Thing::from(("book", "blaze"))))
        .content;

    insta::assert_snapshot!(&cursive_relation.fine_tune_params());
    insta::assert_snapshot!(&cursive_relation.to_raw());
}

#[test]
fn multiplication_tests3() {
    let relation = Student::schema()
        .writes__(
            StudentWritesBook::schema()
                .timeWritten
                .equal_to(Duration::from_secs(343)),
        )
        .book(Empty)
        .content;

    assert_eq!(
        relation.fine_tune_params(),
        "->writes[WHERE timeWritten = $_param_00000001]->book.content".to_string()
    )
}
