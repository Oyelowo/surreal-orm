/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(non_snake_case)]
use chrono::{DateTime, NaiveDateTime, Utc};
use insta;
use std::time::Duration;
use surrealdb::sql;
use surrealdb::{engine::local::Mem, Surreal};
use surrealdb_models::{
    company_schema, user_schema, writes_schema, AlienVisitsPlanet, Blog, Book, Company,
    CompanyLikeUser, Student, StudentLiksBook, StudentWritesBlog, StudentWritesBook, User,
};
use surrealdb_orm::statements::{create, insert, order, relate, select, select_value};
use surrealdb_orm::*;

#[test]
fn can_get_structs_relations() {
    let relations_aliases = Student::get_fields_relations_aliased();
    assert_eq!(
        relations_aliases
            .into_iter()
            .map(|r| r.to_raw().build())
            .collect::<Vec<_>>()
            .join(", "),
        "->writes->book AS writtenBooks, ->writes->blog AS blogsssss"
    );
}

#[tokio::test]
async fn should_not_contain_error_when_valid_id_use_in_connection() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let student_id1 = Student::create_id("1");
    let student_id2 = Student::create_id("2");
    let book_id1 = Book::create_id("1");
    let book_id2 = Book::create_id("2");
    let blog_id = Blog::create_id("1");

    let write_book = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    let write_book2 = StudentWritesBook {
        time_written: Duration::from_secs(923),
        ..Default::default()
    };

    let write_blog = StudentWritesBlog {
        time_written: Duration::from_secs(47),
        count: 24,
        ..Default::default()
    };

    // Student 1 writes book2
    let relation = relate(Student::with(&student_id1).writes__(Empty).book(&book_id2))
        .content(write_book.clone())
        .parallel();

    assert_eq!(relation.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(relation.get_errors(), errors);

    let result = relation.load_all_links()?.get_one(db.clone()).await?;

    assert!(result.clone().id.to_string().starts_with("writes:"),);

    assert_eq!(
        result.clone().in_.get_id().unwrap().to_string(),
        "student:⟨1⟩"
    );
    assert_eq!(result.clone().out.get_id().unwrap().to_string(), "book:⟨2⟩");
    let id = result.clone().id.to_string();
    assert_eq!(
        serde_json::to_string(&result).unwrap(),
        format!("{{\"id\":\"{id}\",\"timeWritten\":{{\"secs\":343,\"nanos\":0}},\"count\":0}}")
    );

    // Student 2 writes book1
    let relation = relate(Student::with(&student_id2).writes__(Empty).book(&book_id1))
        .content(write_book2)
        .parallel();

    assert_eq!(relation.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(relation.get_errors(), errors);

    let result = relation.load_all_links()?.get_one(db.clone()).await?;

    assert!(result.clone().id.to_string().starts_with("writes:"),);

    assert_eq!(
        result.clone().in_.get_id().unwrap().to_string(),
        "student:⟨2⟩"
    );
    assert_eq!(result.clone().out.get_id().unwrap().to_string(), "book:⟨1⟩");
    let id = result.clone().id.to_string();
    assert_eq!(
        serde_json::to_string(&result).unwrap(),
        format!("{{\"id\":\"{id}\",\"timeWritten\":{{\"secs\":923,\"nanos\":0}},\"count\":0}}")
    );

    // Student 2 writes blog1
    let writes_schema::Writes { timeWritten, .. } = StudentWritesBlog::schema();
    // Using the set method
    relate(Student::with(&student_id2).writes__(Empty).blog(&blog_id))
        // .set(updater(timeWritten).equal(sql::Duration::from(Duration::from_secs(47))))
        .content(write_blog)
        .parallel();
    let writes_schema::Writes { count, .. } = StudentWritesBlog::schema();
    let relation =
        relate::<StudentWritesBlog>(Student::with(&student_id2).writes__(Empty).blog(&blog_id))
            .set(updater(count).increment_by(545))
            .set(updater(timeWritten).equal(sql::Duration::from(Duration::from_secs(47))))
            .parallel();

    assert_eq!(relation.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(relation.get_errors(), errors);

    let result = relation.load_all_links()?.return_one(db.clone()).await?;

    assert!(result
        .clone()
        .unwrap()
        .id
        .to_string()
        .starts_with("writes:"));

    assert_eq!(
        result.clone().unwrap().in_.get_id().unwrap().to_string(),
        "student:⟨2⟩"
    );
    assert_eq!(
        result.clone().unwrap().out.get_id().unwrap().to_string(),
        "blog:⟨1⟩"
    );
    let id = result.clone().unwrap().id.to_string();
    assert_eq!(
        serde_json::to_string(&result).unwrap(),
        format!("{{\"id\":\"{id}\",\"timeWritten\":{{\"secs\":47,\"nanos\":0}},\"count\":545}}")
    );

    let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();

    let result: Vec<StudentWritesBook> = select(All)
        .from(StudentWritesBook::table_name())
        .order_by(order(timeWritten).asc())
        .return_many(db.clone())
        .await?;

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].time_written, Duration::from_secs(47));
    assert_eq!(result[0].in_.get_id().unwrap().to_string(), "student:⟨2⟩");
    assert_eq!(result[0].out.get_id().unwrap().to_string(), "blog:⟨1⟩");

    assert_eq!(result[1].time_written, Duration::from_secs(343));
    assert_eq!(result[1].in_.get_id().unwrap().to_string(), "student:⟨1⟩");
    assert_eq!(result[1].out.get_id().unwrap().to_string(), "book:⟨2⟩");

    assert_eq!(result[2].time_written, Duration::from_secs(923));
    assert_eq!(result[2].in_.get_id().unwrap().to_string(), "student:⟨2⟩");
    assert_eq!(result[2].out.get_id().unwrap().to_string(), "book:⟨1⟩");

    Ok(())
}

// -- Add a graph edge between multiple specific users and devs
// LET $from = (SELECT users FROM company:surrealdb);
// LET $devs = (SELECT * FROM user WHERE tags CONTAINS 'developer');
// RELATE $from->like->$devs SET time.connected = time::now();
//
#[tokio::test]
async fn can_relate_subquery_to_subquery_relate_with_queries() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    // Create a bunch of users as deleoper
    let generated_developers = (0..10)
        .into_iter()
        .map(|i| {
            let user = User {
                name: format!("user{}", i),
                tags: vec!["developer".to_string()],
                ..Default::default()
            };
            user
        })
        .collect::<Vec<_>>();

    let oyelowo = User {
        id: User::create_simple_id(),
        name: "oyelowo".to_string(),
        tags: vec!["developer".to_string()],
        ..Default::default()
    };
    let _ = create(oyelowo).return_one(db.clone()).await;
    let devs = insert(generated_developers).return_many(db.clone()).await?;
    let sample = devs.into_iter().take(2).collect::<Vec<_>>();

    // Create company
    let codebreather_coy = Company {
        id: Company::create_simple_id(),
        name: "codebreather".to_string(),
        users: LinkMany::from(sample),
        ..Default::default()
    };
    let codebreather = create(codebreather_coy)
        .return_one(db.clone())
        .await?
        .unwrap();

    let user_schema::User { tags, .. } = User::schema();
    let company_schema::Company { users, .. } = Company::schema();

    // select users from company
    let from_statement = select_value(users).from(codebreather.id);
    // select devs
    let devs_statement = select(All)
        .from(User::table_name())
        .where_(tags.contains("developer"));

    let time = CompanyLikeUser::schema().time();
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);

    // Relate fromstate -> like-> devs
    let relation = relate::<CompanyLikeUser>(
        Company::with(from_statement)
            .like__(Empty)
            .user(devs_statement),
    )
    .set(updater(time.connected).equal(sql::Datetime(dt)));

    assert_eq!(relation.get_errors().len(), 0);

    // Variable binding and deserialaization not yet working properly.
    // TODO: Fix this. It should be possible to bind variables to the relation
    // with subqueries and then deserialize the result.
    // The problem is that the variable binding is not being deserialized properly
    // so i cannot use the return methods for now. I am turning it back into
    // a raw query for now.
    relation.to_raw().run(db.clone()).await?;
    // Expected
    // let result = relation.return_many(db.clone()).await?;

    let result = select(All)
        .from(CompanyLikeUser::table_name())
        .return_many::<CompanyLikeUser>(db.clone())
        .await?;
    assert_eq!(result.len(), 22);

    Ok(())
}

#[test]
fn test_relation_graph_with_alias() {
    let student_id = Student::create_id("oyelowo");
    let book_id = Book::create_id(vec!["The", "Alchemist"]);

    let aliased_connection = Student::with(student_id)
        .writes__(Empty)
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->writes->$_param_00000002 AS writtenBooks"
    );

    assert_eq!(
        aliased_connection.clone().to_raw().build(),
        "student:oyelowo->writes->book:['The', 'Alchemist'] AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn test_recursive_edge_to_edge_connection_as_supported_in_surrealql() {
    // This allows for pattern like this:
    // -- Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
    // SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;

    let student_id = Student::create_id("oyelowo");
    let book_id = Book::create_id("2");
    let likes = StudentLiksBook::table_name();
    let writes = StudentWritesBook::table_name();
    let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();

    let aliased_connection = Student::with(student_id)
        .writes__(Empty)
        .writes__(Empty)
        .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->writes->writes->(writes, writes, likes  WHERE timeWritten <= $_param_00000002)->$_param_00000003 AS writtenBooks"
    );

    assert_eq!(
       aliased_connection.clone().to_raw().build(),
        "student:oyelowo->writes->writes->(writes, writes, likes  WHERE timeWritten <= 50)->book:⟨2⟩ AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn test_any_edge_filter() {
    let student_id = Student::create_id("oye");
    let book_id = Book::create_id("mars");
    let likes = StudentLiksBook::table_name();
    let visits = AlienVisitsPlanet::table_name();
    let writes_schema::Writes { timeWritten, .. } = StudentWritesBook::schema();

    let aliased_connection = Student::with(student_id)
        .writes__(any_other_edges(&[visits, likes]).where_(timeWritten.less_than_or_equal(50)))
        .book(book_id)
        .__as__(Student::aliases().writtenBooks);

    assert_eq!(
        aliased_connection.fine_tune_params(),
        "$_param_00000001->(writes, visits, likes  WHERE timeWritten <= $_param_00000002)->$_param_00000003 AS writtenBooks"
    );

    assert_eq!(
        dbg!(aliased_connection.clone()).to_raw().build(),
        "student:oye->(writes, visits, likes  WHERE timeWritten <= 50)->book:mars AS writtenBooks"
    );

    assert_eq!(aliased_connection.get_errors().len(), 0);
    let errors: Vec<String> = vec![];
    assert_eq!(aliased_connection.get_errors(), errors);
}

#[test]
fn should_contain_error_when_invalid_id_use_in_connection() {
    let student_id = Student::create_id("oye");
    let book_id = Book::create_id("mars");

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };

    // Book id used with student schema, while student_id used for book. This should generate
    // two errors
    let relate_statement = relate(Student::with(&book_id).writes__(Empty).book(&student_id))
        .content(write.clone())
        .return_type(ReturnType::Before)
        .parallel();

    assert_eq!(relate_statement.get_errors().len(), 2);
    assert_eq!(
        relate_statement.get_errors(),
        vec![
            "invalid id book:mars. Id does not belong to table student",
            "invalid id student:oye. Id does not belong to table book"
        ]
    );
}

#[tokio::test]
async fn relate_query() -> surrealdb_orm::SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("failed to use db");
    let student_id = Student::create_id("oyelowo");
    let book_id = Book::create_id("kivi");

    let write = StudentWritesBook {
        time_written: Duration::from_secs(343),
        ..Default::default()
    };
    let write_id = write.id.clone();

    let relate_simple = relate(Student::with(student_id).writes__(E).book(book_id)).content(write);
    assert_eq!(
        relate_simple.to_raw().build(),
        format!("RELATE student:oyelowo->writes->book:kivi CONTENT {{ count: 0, id: {write_id}, timeWritten: {{ nanos: 0, secs: 343 }} }} ;")
    );

    // // You can use return one method and it just returns the single object
    let relate_simple_object = relate_simple.return_one(db.clone()).await?;
    assert_eq!(
        relate_simple_object.clone().unwrap().time_written,
        Duration::from_secs(343)
    );
    assert_eq!(
        relate_simple_object
            .clone()
            .unwrap()
            .in_
            .get_id()
            .unwrap()
            .to_string(),
        "student:oyelowo"
    );

    assert_eq!(
        relate_simple_object
            .unwrap()
            .out
            .get_id()
            .unwrap()
            .to_string(),
        "book:kivi"
    );

    let relate_simple_array = relate_simple.return_many(db.clone()).await?;
    assert_eq!(relate_simple_array.len(), 1);
    assert_eq!(
        relate_simple_array[0].clone().time_written,
        Duration::from_secs(343)
    );
    assert_eq!(
        relate_simple_array[0]
            .clone()
            .in_
            .get_id()
            .unwrap()
            .to_string(),
        "student:oyelowo"
    );
    assert_eq!(
        relate_simple_array[0]
            .clone()
            .out
            .get_id()
            .unwrap()
            .to_string(),
        "book:kivi"
    );

    Ok(())
}

#[tokio::test]
async fn relate_query_with_sub_query() -> surrealdb_orm::SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("failed to use db");

    let write = StudentWritesBook {
        time_written: Duration::from_secs(52),
        ..Default::default()
    };
    let statement = relate(
        Student::with(select(All).from(Student::get_table_name()))
            .writes__(E)
            .book(
                select(All).from(Book::get_table_name()), // .where_(Book::schema().title.like("Oyelowo")),
            ),
    )
    .content(write.clone());

    assert_eq!(statement.get_errors().len(), 0);
    assert_eq!(statement.get_bindings().len(), 1);
    assert_eq!(
        statement.fine_tune_params(),
        "RELATE (SELECT * FROM student)->writes->(SELECT * FROM book) CONTENT $_param_00000001 ;"
    );
    let write_id = write.get_id();
    assert_eq!(
        statement.to_raw().build(),
        format!("RELATE (SELECT * FROM student)->writes->(SELECT * FROM book) CONTENT {{ count: 0, id: {write_id}, timeWritten: {{ nanos: 0, secs: 52 }} }} ;")
    );

    let result = statement.return_many(db.clone()).await?;
    assert_eq!(result.len(), 0);

    insta::assert_debug_snapshot!(result);
    Ok(())
}
