use std::fmt::Display;

pub struct SurId((String, String));

impl Display for SurId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SurId((table_name, id_part)) = self;
        f.write_fmt(format_args!("{table_name}:{id_part}"))
    }
}

// impl From<String> for SurIdComplex {
//     fn from(value: String) -> Self {
//         Self::from(value).0
//     }
// }

impl SurId {
    fn id(self) -> (String, String) {
        self.0
    }
    pub fn from_string(str: String) -> (String, String) {
        Self::from(str).0
    }
}

// impl From<SurId> for &str {
//     fn from(value: SurId) -> Self {
//         let SurId((table_name, id_part)) = value;
//         let sur_str = format!("{table_name}:{id_part}").as_str();
//         sur_str
//     }
// }

impl From<SurId> for String {
    fn from(value: SurId) -> Self {
        let SurId((table_name, id_part)) = value;
        format!("{table_name}:{id_part}",)
    }
}

impl From<SurId> for (String, String) {
    fn from(value: SurId) -> Self {
        value.0
    }
}

impl From<&str> for SurId {
    fn from(value: &str) -> Self {
        let mut spl = value.split(':');
        match (spl.next(), spl.next(), spl.next()) {
            (Some(table), Some(id), None) => Self((table.into(), id.into())),
            _ => panic!(),
        }
    }
}

impl From<String> for SurId {
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
