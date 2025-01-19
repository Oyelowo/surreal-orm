use serde::Serialize;
use surreal_orm::{pick, Pickable};

#[derive(Pickable, Debug, Serialize)]
struct Person<'a, T: 'a, U: 'a> {
    name: String,
    age: u8,
    some: &'a T,
    another: &'a U,
}

pick!(NewPersonWithUnusedTypeGenericsSkipped, Person<'a,_,_> as PersonPickable, [name, age]);
pick!(NewPerson, Person<'a,T,U> as PersonPickable, [name, age]);

pick! {
    #[derive(Serialize)]
    NewPersonWithAttributes, Person<'a,_,_> as PersonPickable,
    [
        #[serde(rename = "name2")]
        name,
        age,
        // #[serde(borrow)]
        // some
    ]
}

// #[pick(OldPerson, [age, num])]
// #[pick(Book, [title, author])]
// struct NewStructThing {
//    extra: String,
//     #[override]
//     age: u32,
// }

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

    let new1 = NewPerson::<'_, u32, &str> {
        name: "Oye".to_string(),
        age: 154,
    };
    println!("{}", new1.name);
    println!("{}", new1.age);

    let new3 = NewPersonWithAttributes {
        name: "Oye".to_string(),
        age: 154,
    };
    println!("{}", new3.name);
}
