pub struct SurIdComplex((String, String));

// impl From<String> for SurIdComplex {
//     fn from(value: String) -> Self {
//         Self::from(value).0
//     }
// }

impl SurIdComplex {
    fn id(self) -> (String, String) {
        self.0
    }
    pub fn from_string(str: String) -> (String, String) {
        Self::from(str).0
    }
}

impl From<SurIdComplex> for (String, String) {
    fn from(value: SurIdComplex) -> Self {
        value.0
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
