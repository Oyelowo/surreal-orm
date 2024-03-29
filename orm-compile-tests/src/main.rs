// August 23, 2023.
// When any of these checks stops compiling, make sure
// to update the corresponding doc tests in the file.
// The doc tests are expected to fail to compile.
// So, we want to be sure that the failures are coming from the right place
// with field attributes and not from imports or other places we do not expect
// or from misspelled attributes names or functions names etc.
// We are mostly testing for invalid attribute combinations here.
// e.g
// define and define_fn should not be used together,
// value and value_fn should not be used together.
// assert and assert_fn should not be used together.
// permissions and permissions_fn should not be used together.
// <attr_name> and <attr_name>_fn should not be used together.
// NOTE: Change this if the logic changes in the future.

mod check1 {
    use surreal_orm_compile_tests::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(type_ = "int", define = "define_field_fn()")]
        age: u8,
    }
}

mod check2 {

    use surreal_orm_compile_tests::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(type_ = "int", define_fn = "define_field_fn")]
        age: u8,
    }
}

mod check3 {

    use surreal_orm_compile_tests::*;

    #[derive(Node, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[surreal_orm(table_name = "student")]
    pub struct Student {
        id: SurrealSimpleId<Self>,
        #[surreal_orm(
            type_ = "int",
            value = "18",
            assert = "assert_fn()",
            permissions = "permissions_fn()"
        )]
        age: u8,
    }
}

fn main() {
    println!("Hello, world!");
}
