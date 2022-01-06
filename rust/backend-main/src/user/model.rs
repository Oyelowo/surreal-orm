// use super::query::User;
// use super::User;
use super::query_user::User;

#[derive(Clone)]
pub struct UserData {
    pub users: Vec<User>,
}

impl UserData {
    pub fn new() -> Self {
        let users = vec![
            User {
                id: 1,
                name: String::from("Oyelowo"),
                age: 114,
                family_count: 75,
            },
            User {
                id: 2,
                name: String::from("Oyedayo"),
                age: 87,
                family_count: 45,
            },
            User {
                id: 3,
                name: String::from("Jupiter"),
                age: 11,
                family_count: 34,
            },
            User {
                id: 4,
                name: String::from("Uranus"),
                age: 14,
                family_count: 4,
            },
            User {
                id: 5,
                name: String::from("Mari"),
                age: 7,
                family_count: 5,
            },
            User {
                id: 6,
                name: String::from("Saul"),
                age: 54,
                family_count: 405,
            },
            User {
                id: 7,
                name: String::from("Olli"),
                age: 93,
                family_count: 162,
            },
        ];

        Self { users }
    }

    pub fn get_user(&self, id: i32) -> User {
        let user = self.users.iter().find(|u| u.id == id).unwrap().clone();
        // let user = self.users.into_iter().find(|u| u.id == id).unwrap();
        user
    }

    pub fn get_users(self) -> Vec<User> {
        self.users.into_iter().collect()
    }

    pub fn _delete_user(&mut self, id: i32) -> User {
        let position = self.users.iter().position(|u| u.id == id).unwrap();

        // let k = self.users.into_iter().find(|u| u.id == id).unwrap();
        self.users.remove(position)
    }
}

// struct UserData2 {
//     users: Rc<RefCell<Vec<User>>>,
// }

// impl UserData2 {
//     fn new(&self) -> Self {
//         let users = vec![
//             User {
//                 id: 1,
//                 name: "Oyelowo",
//                 age: 114,
//                 family_count: 75,
//             },
//             User {
//                 id: 2,
//                 name: "Oyedayo",
//                 age: 87,
//                 family_count: 45,
//             },
//             User {
//                 id: 3,
//                 name: "Jupiter",
//                 age: 11,
//                 family_count: 34,
//             },
//             User {
//                 id: 4,
//                 name: "Uranus",
//                 age: 14,
//                 family_count: 4,
//             },
//             User {
//                 id: 5,
//                 name: "Mari",
//                 age: 7,
//                 family_count: 5,
//             },
//             User {
//                 id: 6,
//                 name: "Saul",
//                 age: 54,
//                 family_count: 405,
//             },
//             User {
//                 id: 7,
//                 name: "Olli",
//                 age: 93,
//                 family_count: 162,
//             },
//         ];

//         return Self { users : Rc::new(RefCell::new(users))};
//     }

//     pub fn get_fake_user(&self, id: i32) -> &User {
//         let k = self.users.borrow().iter().find(|u| u.id == id).unwrap();
//         // let k = self.users.into_iter().find(|u| u.id == id).unwrap();
//         *k
//     }

//     pub fn get_all_users(&self) -> Vec<&User> {
//         self.users.borrow().iter().collect()
//     }

//         pub fn remove_fake_user(&self, id: i32) -> User {
//             let position = self.users.borrow().iter().position(|u| u.id == id).unwrap();

//             let k = self.users.get_mut().remove(position);

//             // let k = self.users.into_iter().find(|u| u.id == id).unwrap();
//             k
//         }

// }

// fn get_fake_user(id: i32) -> User {
//     let users = get_all_users();
//     let k = users.into_iter().find(|u| u.id == id).unwrap();
//     k
// }

// fn get_all_users() -> Vec<User> {
//     return vec![
//         User {
//             id: 1,
//             name: "Oyelowo",
//             age: 114,
//             family_count: 75,
//         },
//         User {
//             id: 2,
//             name: "Oyedayo",
//             age: 87,
//             family_count: 45,
//         },
//         User {
//             id: 3,
//             name: "Jupiter",
//             age: 11,
//             family_count: 34,
//         },
//         User {
//             id: 4,
//             name: "Uranus",
//             age: 14,
//             family_count: 4,
//         },
//         User {
//             id: 5,
//             name: "Mari",
//             age: 7,
//             family_count: 5,
//         },
//         User {
//             id: 6,
//             name: "Saul",
//             age: 54,
//             family_count: 405,
//         },
//         User {
//             id: 7,
//             name: "Olli",
//             age: 93,
//             family_count: 162,
//         },
//     ];
// }
