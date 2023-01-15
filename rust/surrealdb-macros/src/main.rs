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
    query::{Foreign, ForeignVec, KeySerializeControl, QueryBuilder},
    Edge, SurrealdbModel,
};
use typed_builder::TypedBuilder;

#[derive(SurrealdbModel, Default, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    handle: String,
    // #[surrealdb(rename = "nawao")]
    first_name: String,
    #[surrealdb(reference_one = "Account", skip_serializing)]
    best_friend: Box<Foreign<Account>>,

    #[surrealdb(reference_one = "Account", skip_serializing)]
    teacher: Box<Foreign<Account>>,

    #[surrealdb(rename = "lastName")]
    another_name: String,
    chess: String,
    nice_poa: String,
    password: String,
    email: String,

    // #[surrealdb(relate(edge="Account_Manage_Project", description="->manage->Account"))]
    #[surrealdb(relate(edge = "AccountManageProject", link = "->manage->Project"))]
    managed_projects: ForeignVec<Project>,
}

struct ForeignWrapper<T>(Foreign<T>);

impl<T> ForeignWrapper<T> {
    fn allow_serialize_value() -> T {
        todo!()
    }

    fn disallow_serialize_value() -> String {
        todo!()
    }
}

#[derive(SurrealdbModel, Default, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Project {
    id: Option<String>,
    title: String,
    // #[surrealdbrelate = "->run_by->Account")]
    #[surrealdb(relate(edge = "AccountManageProject", link = "<-manage<-Account"))]
    account: ForeignVec<Account>,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
#[surrealdb(relation_name = "manage")]
struct AccountManageProject {
    id: Option<String>,
    #[surrealdb(reference_one = "Account", skip_serializing, rename = "in")]
    _in: Account,
    // r#in: Account,
    #[surrealdb(reference_one = "Project", skip_serializing)]
    out: Project,
    when: String,
    destination: String,
}

#[derive(SurrealdbModel, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
#[surrealdb(rename_all = "camelCase")]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    first_name: String,
    #[surrealdb(reference_one = "Course", skip_serializing)]
    course: Ref<Course>,
}

#[derive(SurrealdbModel, TypedBuilder, Default, Serialize, Deserialize, Debug, Clone)]
#[surrealdb(rename_all = "camelCase")]
pub struct Course {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    id: Option<String>,
    title: String,
}

mod ref_mod {
    use super::*;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(untagged)]
    enum Reference<V: SurrealdbModel> {
        Id(String),
        FetchedValue(V),
        None,
    }

    impl<V: SurrealdbModel> Default for Reference<V> {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone, Default)]
    pub struct Ref<V: SurrealdbModel>(Reference<V>);

    impl<V> Ref<V>
    where
        V: SurrealdbModel,
    {
        /// .
        ///
        /// # Panics
        ///
        /// Panics if .
        pub fn from_model<M: SurrealdbModel>(model: M) -> Self {
            let x = model.get_key();
            Self(Reference::Id(x.unwrap()))
        }
    }
}
use ref_mod::Ref;

static DB: Surreal<Db> = Surreal::init();

struct SurId(String);
struct SurIdComplex((String, String));

impl SurIdComplex {
    fn id(self) -> (String, String) {
        self.0
    }
    fn from_string(str: String) -> (String, String) {
        Self::from(str).0
    }
}

impl From<SurIdComplex> for (String, String) {
    fn from(value: SurIdComplex) -> Self {
        value.0
    }
}

impl DerefMut for SurIdComplex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for SurIdComplex {
    type Target = (String, String);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SurIdComplex {
    fn from(value: String) -> Self {
        let mut spl = value.split(':');
        match (spl.next(), spl.next(), spl.next()) {
            (Some(table), Some(id), None) => Self((table.into(), id.into())),
            _ => panic!(),
        }
    }
}

// impl IntoResource for SurId {
//     fn into_resource(self) -> Result<surrealdb::opt::Resource> {
//         todo!()
//     }
// }

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
    let cf = Ref::from_model(cs.unwrap());
    // let cf = Ref::Id(cs.unwrap().id.unwrap().into());

    // println!(
    //     "cours {}",
    //     serde_json::to_string(&cf.value().unwrap().id).unwrap()
    // );

    let stu = Student::builder()
        .first_name("dayo".into())
        .course(cf)
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

    let sql_query = QueryBuilder::new()
        .select("*")
        .from(stud1.clone().id.unwrap())
        .fetch(Student::get_schema().course.identifier)
        .build();
    println!("SQL {sql_query}");
    let mut sql_q_result = DB.query(sql_query).await.unwrap();
    let sql_q: Option<Student> = sql_q_result.take(0)?;

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
