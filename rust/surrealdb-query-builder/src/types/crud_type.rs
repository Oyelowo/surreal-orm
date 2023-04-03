use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug)]
pub enum CrudType {
    Create,
    Select,
    Update,
    Delete,
}

impl Display for CrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crud_type = match self {
            CrudType::Create => "create",
            CrudType::Select => "select",
            CrudType::Update => "update",
            CrudType::Delete => "delete",
        };
        write!(f, "{}", crud_type)
    }
}
