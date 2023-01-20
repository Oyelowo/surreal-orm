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
    use surrealdb_macros::{
        node_builder::{NodeBuilder as NodeBuilder2, ToNodeBuilder as ToNodeBuilder2},
        query_builder::query,
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

    pub enum Clause {
        All,
        Where(String),
        // Change to SurId
        Id(String),
    }
    mod student_schema {
        use super::{
            blog_schema::Blog, book_schema::Book, juice_schema::Juice, water_schema::Water, Clause,
            *,
        };

        #[derive(Debug, Default)]
        pub struct Student {
            // id: String
            id: String,
            // name: String
            name: String,
            // ->writes->book
            book_written: String,
            // ->writes->blog
            blog_written: String,
            // ->drinks->water
            drunk_water: String,
            // ->drinks->juice
            drunk_juice: String,
            pub ___________store: String,
        }

        impl Student {
            pub fn traverse() -> Self {
                Self {
                    id: "id".into(),
                    name: "foreign".into(),
                    blog_written: "blog_written".into(),
                    book_written: "book_written".into(),
                    drunk_water: "drunk_water".into(),
                    drunk_juice: "drunk_juice".into(),
                    ___________store: "".to_string(),
                }
            }

            pub fn writes__(&self, clause: Clause) -> Writes {
                let mut xx = Writes::default();
                xx.__________store.push_str(self.___________store.as_str());
                let pp = get_clause(clause, "writes");
                xx.__________store
                    .push_str(format!("->writes{pp}->").as_str());
                xx
            }

            pub fn drinks__(&self, clause: Clause) -> Drinks {
                let mut xx = Drinks::default();
                xx.__________store.push_str(self.___________store.as_str());
                let pp = get_clause(clause, "drinks");
                xx.__________store
                    .push_str(format!("->drinks{pp}->").as_str());
                xx
            }
            pub fn done(self) -> String {
                self.___________store.clone()
            }
        }

        #[derive(Debug, Default)]
        pub struct Writes {
            id: String,
            r#in: String,
            out: String,
            time_written: String,
            __________store: String,
        }

        impl Writes {
            pub fn new() -> Self {
                Self {
                    id: "id".into(),
                    r#in: "in".into(),
                    out: "out".into(),
                    time_written: "time_written".into(),
                    __________store: "".into(),
                }
            }

            pub fn book(&self, clause: Clause) -> Book {
                let mut xx = Book::default();
                xx.__________store.push_str(self.__________store.as_str());
                let pp = get_clause(clause, "book");
                xx.__________store.push_str(format!("book{pp}").as_str());
                xx
            }

            pub fn blog(&self, clause: Clause) -> Blog {
                let mut xx = Blog::default();
                xx.______________store
                    .push_str(self.__________store.as_str());
                let pp = get_clause(clause, "blog");
                xx.______________store
                    .push_str(format!("blog{pp}").as_str());

                xx
            }
        }

        #[derive(Debug, Default)]
        pub struct Drinks {
            id: String,
            r#in: String,
            out: String,
            rate: String,
            __________store: String,
        }

        impl Drinks {
            pub fn new() -> Self {
                Self {
                    id: "id".into(),
                    r#in: "in".into(),
                    out: "out".into(),
                    rate: "time_written".into(),
                    __________store: "".into(),
                }
            }

            pub fn water(&self, clause: Clause) -> Water {
                let mut xx = Water::default();
                xx.______________store
                    .push_str(self.__________store.as_str());
                let pp = get_clause(clause, "water");
                xx.______________store
                    .push_str(format!("water{pp}").as_str());

                xx
            }
            pub fn juice(&self, clause: Clause) -> Juice {
                let mut xx = Juice::default();
                xx.______________store
                    .push_str(self.__________store.as_str());
                let pp = get_clause(clause, "juice");
                xx.______________store
                    .push_str(format!("juice{pp}").as_str());

                xx.flavor.push_str(xx.______________store.as_str());
                xx.flavor.push_str(".flavor");
                xx.maker.push_str(xx.______________store.as_str());
                xx.maker.push_str(".maker");
                xx
            }
        }
    }
    pub fn get_clause(clause: Clause, table_name: &'static str) -> String {
        let pp = match clause {
            Clause::All => "".into(),
            Clause::Where(where_clause) => {
                if !where_clause.to_lowercase().starts_with("where") {
                    panic!("Invalid where clause, must start with `WHERE`")
                }
                format!("[{where_clause}]")
            }
            Clause::Id(id) => {
                if !id
                    .to_lowercase()
                    .starts_with(format!("{table_name}:").as_str())
                {
                    // let xx = format!("invalid id {id}. Id does not belong to table {table_name}")
                    //     .as_str();
                    panic!("invalid id {id}. Id does not belong to table {table_name}")
                }
                format!("[WHERE id = {id}]")
            }
        };
        pp
    }

    struct Cond(String);

    impl ::std::fmt::Display for Cond {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}", self.0))
        }
    }
    mod blog_schema {
        #[derive(Debug, Default)]
        pub struct Blog {
            id: String,
            intro: String,
            poster: String,
            comments: String,
            pub ______________store: String,
        }
    }
    mod water_schema {
        #[derive(Debug, Default)]
        pub struct Water {
            id: String,
            source: String,
            river: String,
            pub ______________store: String,
        }
    }
    mod juice_schema {
        #[derive(Debug, Default)]
        pub struct Juice {
            pub id: String,
            pub maker: String,
            pub flavor: String,
            pub ______________store: String,
        }

        impl Juice {
            pub fn done(self) -> String {
                self.______________store
            }
        }
    }
    mod book_schema {
        use super::{blog_schema::Blog, student_schema::Student, Clause, *};

        #[derive(Debug, Default)]
        pub struct Book {
            id: String,
            title: String,
            // <-writes<-Student
            writer: String,
            chapters: String,
            pub __________store: String,
        }

        #[derive(Debug, Default)]
        pub struct Writes {
            id: String,
            time_written: String,
            ___________store: String,
        }

        impl Writes {
            pub fn new() -> Self {
                Self {
                    id: "".into(),
                    time_written: "".into(),
                    ___________store: "".into(),
                }
            }

            pub fn student(&self, clause: Clause) -> Student {
                let mut xx = Student::default();
                xx.___________store.push_str(self.___________store.as_str());
                let pp = get_clause(clause, "student");
                xx.___________store
                    .push_str(format!("student{pp}").as_str());
                xx
            }

            // fn blog(&self, cond: Clause) -> Blog {
            //     let mut xx = Blog::default();
            //     xx.store.push_str(self.store.as_str());
            //     let pp = get_clause(cond, "blog");
            //     xx.store.push_str(format!("blog{pp}").as_str());
            //     xx
            // }
        }
        impl Book {
            /// .
            pub fn __writes(&self, clause: Clause) -> Writes {
                let mut xx = Writes::default();
                xx.___________store.push_str(self.__________store.as_str());
                let pp = get_clause(clause, "writes");
                xx.___________store
                    .push_str(format!("<-writes{pp}<-").as_str());
                xx
            }

            pub fn done(self) -> String {
                self.__________store.clone()
            }
        }
    }

    mod chapters_schema {
        use super::DbField;

        #[derive(Debug, Default)]
        struct Chapters {
            id: DbField,
            verse: DbField,
            __________store: String,
            // writer: Relate,
        }
    }
    pub fn nama() {
        // Student::new().book_written().chapters().verse
        // DbField("df".into())
        // "".contains_not()
        // let rela = Student::new()
        //     .book_written_cond(Cond("WHERE pages > 5".into()))
        //     .writer();
        // println!("rela...{:?}", rela.store);

        let rela = student_schema::Student::traverse()
            .writes__(Clause::Where(
                query()
                    .and_where("pages > 5")
                    .and("time_done = yesterday")
                    .build(),
            ))
            .book(Clause::Id("book:akkaka".into()))
            .__writes(Clause::All)
            .student(Clause::Id("student:lowo".into()))
            .writes__(Clause::All)
            .book(Clause::All)
            .__writes(Clause::All)
            .student(Clause::All)
            // .done();
            .drinks__(Clause::All)
            .juice(Clause::All)
            .maker;
        // .done();
        // .done();

        println!("rela...{:?}", rela);

        let rela = student_schema::Student::traverse()
            .writes__(Clause::Where(
                query()
                    .and_where("pages > 5")
                    .and("time_done = yesterday")
                    .build(),
            ))
            .blog(Clause::Id("blog:akkaka".into()));

        println!("rela...{:?}", rela.______________store);
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
    // let xx =S
    schema::nama();
}

// LET $from = (SELECT users FROM company:surrealdb);
// LET $devs = (SELECT * FROM user WHERE tags CONTAINS 'developer');
// RELATE $from->like->$devs SET time.connected = time::now();
// struct Company {
//   users: LinkMany<User>
// }
//
// struct User {
//     tags: Vec<String>,
//     company: LinkOne<Company>,
//     companies: LinkMany<Company>,
// }
// RELATE User[where company.id == company:surrealdb]->like->User[where tags contains 'developer']
//
//
/* #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
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
} */
// Account::with_id(SuId(""))

/*
========RELATE===========
 * -- Add a graph edge between two specific records
RELATE user:tobie->write->article:surreal SET time.written = time::now();

-- Add a graph edge between multiple specific users and devs
LET $from = (SELECT users FROM company:surrealdb);
LET $devs = (SELECT * FROM user WHERE tags CONTAINS 'developer');
RELATE $from->like->$devs SET time.connected = time::now();/

RELATE user:tobie->write->article:surreal CONTENT {
    source: 'Apple notes',
    tags: ['notes', 'markdown'],
    time: {
        written: time::now(),
    },
};

========SELECT===========
-- Select a remote field from connected out graph edges
SELECT ->like->friend.name AS friends FROM person:tobie;


-- Conditional filtering based on graph edges
SELECT * FROM profile WHERE count(->experience->organisation) > 3;

SELECT * FROM person WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
PREFERRED: SELECT * FROM person WHERE ->knows[WHERE influencer = true]->person

#[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Person {
    #[surrealdb(relate(edge = "PersonKnowsPerson", link = "->knows->Person"))]
   known_persons: Relate<Person>
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize)]
#[surrealdb(relation_name = "knows")]
struct PersonKnowsPerson {
    id: Option<String>,
    #[surrealdb(link_one = "Person", skip_serializing)]
    r#in: LinkOne<Person>,
    #[surrealdb(link_one = "Person", skip_serializing)]
    out: LinkOne<Person>,
    influencer: bool,
}

SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL;


========DELETE===========
// DELETE person WHERE ->knows->person->(knows WHERE influencer = false) TIMEOUT 5s;

========UPDATE===========
// UPDATE person SET important = true WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
// PREFERRED: UPDATE person SET important = true WHERE ->knows->person[WHERE influencer = true] TIMEOUT 5s;
*/
