#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(inherent_associated_types)]
#![feature(generic_const_exprs)]

use _core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
// #![feature(inherent_associated_types)]
// #![feature(const_mut_refs)]
// use serde::{Deserialize, Serialize};
// use surreal_simple_querybuilder::prelude::*;
use static_assertions::*;
use surrealdb::{
    engine::local::{Db, Mem},
    opt::IntoResource,
    sql::Id,
    Result, Surreal,
};
// const_assert!("oylelowo".as_str().len() > 3);
// assert_fields!(Account_Manage_Project: r#in, out);

use surrealdb_macros::{
    links::{LinkMany, LinkOne, LinkSelf, Relate},
    model_id::SurIdComplex,
    node_builder::{NodeBuilder as NodeBuilder2, ToNodeBuilder as ToNodeBuilder2},
    query_builder::{query, ToNodeBuilder},
    Edge, SurrealdbModel,
};
use typed_builder::TypedBuilder;

#[derive(SurrealdbModel, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    handle: String,
    // #[surrealdb(rename = "nawao")]
    first_name: String,
    #[surrealdb(link_self = "Account", skip_serializing)]
    best_friend: LinkSelf<Account>,

    #[surrealdb(link_self = "Account", skip_serializing)]
    teacher: LinkSelf<Account>,

    #[surrealdb(rename = "lastName")]
    another_name: String,
    chess: String,
    nice_poa: String,
    password: String,
    email: String,

    // #[surrealdb(relate(edge="Account_Manage_Project", description="->manage->Account"))]
    #[surrealdb(relate(edge = "AccountManageProject", link = "->manage->Project"))]
    managed_projects: Relate<Project>,
}

#[derive(SurrealdbModel, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Project {
    id: Option<String>,
    title: String,
    // #[surrealdbrelate = "->run_by->Account")]
    #[surrealdb(relate(edge = "AccountManageProject", link = "<-manage<-Account"))]
    account: Relate<Account>,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
#[derive(SurrealdbModel, Debug, Serialize, Deserialize)]
#[surrealdb(relation_name = "manage")]
struct AccountManageProject {
    id: Option<String>,
    #[surrealdb(link_one = "Account", skip_serializing)]
    // #[serde(rename = "in")]
    // _in: LinkOne<Account>,
    r#in: LinkOne<Account>,
    #[surrealdb(link_one = "Project", skip_serializing)]
    out: LinkOne<Project>,
    when: String,
    destination: String,
}

#[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    first_name: String,

    #[surrealdb(link_one = "Course", skip_serializing)]
    course: LinkOne<Course>,

    #[surrealdb(link_many = "Course", skip_serializing)]
    #[serde(rename = "lowo")]
    all_semester_courses: LinkMany<Course>,

    #[surrealdb(relate(edge = "StudentWritesBlog", link = "->writes->Blog"))]
    written_blogs: Relate<Blog>,
}

impl Student {
    fn relate_writes_blog() -> String {
        Student::get_schema()
            .writtenBlogs
            .as_alias(Student::get_schema().writtenBlogs().to_string().as_str())
    }
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Clone)]
#[surrealdb(relation_name = "writes")]
struct StudentWritesBlog {
    id: Option<String>,
    #[surrealdb(link_one = "Student")]
    r#in: LinkOne<Student>,
    #[surrealdb(link_one = "Blog")]
    out: LinkOne<Blog>,
    when: String,
    destination: String,
}

#[derive(SurrealdbModel, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
#[surrealdb(rename_all = "camelCase")]
pub struct Blog {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    title: String,
    #[serde(skip_serializing)]
    content: String,
}

#[derive(SurrealdbModel, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
#[surrealdb(rename_all = "camelCase")]
pub struct Course {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    title: String,
}

::static_assertions::assert_type_eq_all!(LinkOne<Course>, LinkOne<Course>);

// use ref_mod::Ref;
// use ref_mod::{LinkMany, Ref as Mana};

static DB: Surreal<Db> = Surreal::init();

#[tokio::main]
async fn idea_main1() -> Result<()> {
    let xx = Blog {
        id: None,
        title: "lowo".to_string(),
        content: "nama".to_string(),
    };
    println!("mowo {}", serde_json::to_string(&xx).unwrap());

    let m = query()
        .select(Student::relate_writes_blog())
        // .select(Student::get_schema().writtenBlogs.as_alias("writtenBlogs"))
        .from(Student::get_schema())
        .build();

    println!("moam {}", m);
    // DB.connect::<Mem>(()).await?;
    // DB.use_ns("namespace").use_db("database").await?;
    //
    // let c = Course::builder()
    //     // .id(Some("Course:math".into()))
    //     .title("Calculuse".into())
    //     .build();
    // let cx: Course = DB.create("Course").content(&c).await.unwrap();
    // println!("cxxxx.....{:?}", cx);
    // let id = Id::from("Course:math");
    // // let idr = id.to_raw();
    //
    // let cs: Option<Course> = DB
    //     .select(SurIdComplex::from_string(cx.clone().id.unwrap()))
    //     // .select(SurIdComplex(("Course".to_string(), cx.id.unwrap()).0))
    //     .await
    //     .unwrap();
    //
    // // let cx: Course = DB.create(("Course", &*"meth")).content(&c).await.unwrap();
    // println!("cssss.....{:?}", cx);
    // // let cf: Foreign<Course> = Foreign::new_value(cx);
    // // let cf = Ref::from_model(cs.clone().unwrap());
    // // let cf1 = Ref::from_model(cs.clone().unwrap());
    // // let cf2 = Ref::from_model(cs.unwrap());
    // // let cf = Ref::Id(cs.unwrap().id.unwrap().into());
    // let cfake = Course::builder()
    //     .id("Course:math".into())
    //     .title("Calculuse".into())
    //     .build();
    //
    // // println!(
    // //     "cours {}",
    // //     serde_json::to_string(&cf.value().unwrap().id).unwrap()
    // // );
    //
    // let cxx = cs.as_ref().unwrap();
    // // let cxx = &cs.unwrap();
    // let stu = Student::builder()
    //     .first_name("dayo".into())
    //     .course(cxx.into())
    //     .all_semester_courses(vec![cxx.into(), cxx.into(), cfake.into()])
    //     .build();
    //
    // let stud1: Student = DB.create("Student").content(&stu).await.unwrap();
    // println!("stud1: stud1 {:?}", stud1);
    // // stud1.course.allow_value_serialize();
    // println!(
    //     "stud1_serjson: stud1serj {}",
    //     serde_json::to_string(&stud1).unwrap()
    // );
    //
    // let stud_select: Option<Student> = DB
    //     .select(SurIdComplex::from_string(stud1.clone().id.unwrap()))
    //     // .select(SurIdComplex(("Course".to_string(), cx.id.unwrap()).0))
    //     .await
    //     .unwrap();
    //
    // let sql_query = query()
    //     .select("*")
    //     .from(stud_select.unwrap().clone().id.unwrap())
    //     // .from(stud1.clone().id.unwrap())
    //     .fetch_many(&[
    //         Student::get_schema().course.identifier,
    //         Student::get_schema().lowo.identifier,
    //         // "allSemesterCourses",
    //     ])
    //     .build();
    // println!("SQL {sql_query}");
    // let mut sql_q_result = DB.query(sql_query).await.unwrap();
    // let sql_q: Option<Student> = sql_q_result.take(0)?;
    //
    // let mmm = sql_q.clone().unwrap().clone().course.value_ref();
    // println!("sqllll {:?}", sql_q);
    // // println!("studselect1: studselect1 {:?}", stud_select);
    // // sql_q
    // //     .clone()
    // //     .unwrap()
    // //     .clone()
    // //     .course
    // //     .allow_value_serialize();
    // // stud_select.clone().unwrap().course.allow_value_serialize();
    // println!(
    //     "stud SELECT_serjson: rj {}",
    //     serde_json::to_string(&sql_q).unwrap()
    // );
    //
    // println!("relate...{}", Student::get_schema().firstName);
    // // cxxxx.....Course { id: Some("Course:ygz4r9w68lls6e9k8fo5"), title: "Calculuse" }
    Ok(())
}
mod schema {
    use std::fmt::Display;
    use surrealdb_macros::node_builder::{
        NodeBuilder as NodeBuilder2, ToNodeBuilder as ToNodeBuilder2,
    };

    #[derive(Debug, Default)]
    pub struct DbField(String);

    impl Display for DbField {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl ToNodeBuilder2 for DbField {}

    // let x = DbField("lowo".into());
    struct Foreign {}
    // For e.g: ->writes->Book as field_name_as_alias_default
    #[derive(Debug, Default)]
    struct Relate {
        // field_name_as_alias_default
        alias_default: String,

        // writes
        edge_name: String,

        // ->writes
        edge_with_start_arrow: String,

        // writes->
        edge_with_end_arrow: String,

        // ->writes->
        edge_with_both_arrows: String,
    }
    #[derive(Debug, Default)]
    struct Student {
        id: String,
        foreign: String,
        book_written: Relate,
        store: String,
    }

    impl Student {
        fn new() -> Self {
            Self {
                id: "id".into(),
                foreign: "foreign".into(),
                book_written: Relate::default(),
                store: "".to_string(),
            }
        }

        fn book_written(&mut self) -> Book {
            self.store.push_str("->writes->Book");
            let mut xx = Book::default();
            xx.store.push_str(self.store.as_str());
            xx.store.push_str("Book");
            // Book::default()
            xx
        }
    }

    #[derive(Debug, Default)]
    struct Book {
        id: String,
        title: String,
        writer: Relate,
        chapters: Relate,
        store: String,
    }

    impl Book {
        fn writer(&mut self) -> Student {
            self.store.push_str("<-writes<-Student");
            let mut xx = Student::default();
            xx.store.push_str(self.store.as_str());
            xx.store.push_str("Student:id");
            // Book::default()
            xx
        }

        fn chapters(&mut self) -> Chapters {
            self.store.push_str("->has->Chapter");
            let mut xx = Chapters::default();
            xx.store.push_str(self.store.as_str());
            xx.store.push_str("Chapter:id");
            // Book::default()
            xx
        }
    }

    #[derive(Debug, Default)]
    struct Chapters {
        id: DbField,
        verse: DbField,
        store: String,
        // writer: Relate,
    }

    pub fn nama() {
        // Student::new().book_written().chapters().verse
        // DbField("df".into())
        // "".contains_not()
        let rela = Student::new().book_written().chapters();
        println!("rela...{:?}", rela);

        let cycle = Student::new()
            .book_written()
            .writer()
            .book_written()
            .writer();
        println!("rela...{:?}", cycle);
        // let rela = Student::new().book_written().chapters();
    }
}
// impl Book {
//     fn writer(&self) -> Student {
//         todo!()
//     }
// }
// Student->writes->Book->has->Chaper
// let rela = Student::new().book_written().chapters();
// // ->writes->Book->
// let xx = Student::new()
//     .book_written()
//     .writer()
//     .book_written()
//     .writer()
//     .book_written()
//     .writer()
//     .book_written();

#[tokio::main]
async fn main() {
    schema::nama();
}
