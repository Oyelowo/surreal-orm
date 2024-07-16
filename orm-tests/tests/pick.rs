use serde::Serialize;
use surreal_orm::{pick, Pickable};

#[derive(Debug, Serialize)]
struct Person<'a, T: 'a, U: 'a> {
    name: String,
    age: u8,
    some: &'a T,
    another: &'a U,
}
#[allow(non_camel_case_types, unused)]
pub trait PersonPickable {
    type name;
    type age;
    type some;
    type another;
}

impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
    type name = String;
    type age = u8;
    type some = &'a T;
    type another = &'a U;
}


pick!(NewPersonWithUnusedTypeGenericsSkipped, Person<'a,_,_> as PersonPickable, [name, age]);
pick!(NewPerson, Person<'a,T,U> as PersonPickable, [name, age]);

fn main() {
    let person = Person {
        name: "Oyelowo".into(),
        age: 25,
        some: &43,
        another: &"kaka",
    };
    println!("{:?}", person);



    let new2 = NewPersonWithUnusedTypeGenericsSkipped {
        name: "Oye".to_string(),
        age: 154,
    };

    println!("{}", new2.name);
    println!("{}", new2.age);

    let new1 = NewPerson::<'_, u32, &str>{
        name: "Oye".to_string(),
        age: 154,
    };
    println!("{}", new1.name);
    println!("{}", new1.age);
}
