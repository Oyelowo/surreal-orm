#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
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
    query_builder::query,
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
    #[surrealdb(link_one = "Account", skip_serializing)]
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

    #[surrealdb(link_one = "Course", skip_serializing)]
    #[serde(rename = "lowo")]
    all_semester_courses: LinkMany<Course>,
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
async fn main() -> Result<()> {
    DB.connect::<Mem>(()).await?;
    DB.use_ns("namespace").use_db("database").await?;

    let c = Course::builder()
        // .id(Some("Course:math".into()))
        .title("Calculuse".into())
        .build();
    let cx: Course = DB.create("Course").content(&c).await.unwrap();
    println!("cxxxx.....{:?}", cx);
    let id = Id::from("Course:math");
    // let idr = id.to_raw();

    let cs: Option<Course> = DB
        .select(SurIdComplex::from_string(cx.clone().id.unwrap()))
        // .select(SurIdComplex(("Course".to_string(), cx.id.unwrap()).0))
        .await
        .unwrap();

    // let cx: Course = DB.create(("Course", &*"meth")).content(&c).await.unwrap();
    println!("cssss.....{:?}", cx);
    // let cf: Foreign<Course> = Foreign::new_value(cx);
    // let cf = Ref::from_model(cs.clone().unwrap());
    // let cf1 = Ref::from_model(cs.clone().unwrap());
    // let cf2 = Ref::from_model(cs.unwrap());
    // let cf = Ref::Id(cs.unwrap().id.unwrap().into());
    let cfake = Course::builder()
        .id("Course:math".into())
        .title("Calculuse".into())
        .build();

    // println!(
    //     "cours {}",
    //     serde_json::to_string(&cf.value().unwrap().id).unwrap()
    // );

    let cxx = cs.as_ref().unwrap();
    // let cxx = &cs.unwrap();
    let stu = Student::builder()
        .first_name("dayo".into())
        .course(cxx.into())
        .all_semester_courses(vec![cxx.into(), cxx.into(), cfake.into()])
        .build();

    let stud1: Student = DB.create("Student").content(&stu).await.unwrap();
    println!("stud1: stud1 {:?}", stud1);
    // stud1.course.allow_value_serialize();
    println!(
        "stud1_serjson: stud1serj {}",
        serde_json::to_string(&stud1).unwrap()
    );

    let stud_select: Option<Student> = DB
        .select(SurIdComplex::from_string(stud1.clone().id.unwrap()))
        // .select(SurIdComplex(("Course".to_string(), cx.id.unwrap()).0))
        .await
        .unwrap();

    let sql_query = query()
        .select("*")
        .from(stud_select.unwrap().clone().id.unwrap())
        // .from(stud1.clone().id.unwrap())
        .fetch_many(&[
            Student::get_schema().course.identifier,
            Student::get_schema().lowo.identifier,
            // "allSemesterCourses",
        ])
        .build();
    println!("SQL {sql_query}");
    let mut sql_q_result = DB.query(sql_query).await.unwrap();
    let sql_q: Option<Student> = sql_q_result.take(0)?;

    let mmm = sql_q.clone().unwrap().clone().course.value();
    println!("sqllll {:?}", sql_q);
    // println!("studselect1: studselect1 {:?}", stud_select);
    // sql_q
    //     .clone()
    //     .unwrap()
    //     .clone()
    //     .course
    //     .allow_value_serialize();
    // stud_select.clone().unwrap().course.allow_value_serialize();
    println!(
        "stud SELECT_serjson: rj {}",
        serde_json::to_string(&sql_q).unwrap()
    );
    Ok(())
    // cxxxx.....Course { id: Some("Course:ygz4r9w68lls6e9k8fo5"), title: "Calculuse" }
}
