/* TODO: Check if parameters can use numbers. And automatically generate and populate for them
Otherwise, force users to provide the param name. e.g */
fn name() {
    let query = QueryBuilder::new()
        .select("*")
        // you can pass the entire model directly to reference its name
        .from(user)
        // or you can access its fields and use the various traits imported from
        // the querybuilding crate to form complex queries
        .filter(user.age.greater_than("10"))
        .and(user.name.equals("'John'"))
        .build();

    enum Param {
        Number(u32),
        String(String),
    }
    let query = QueryBuilder::new()
        .select("*")
        // you can pass the entire model directly to reference its name
        .from(user)
        // or you can access its fields and use the various traits imported from
        // the querybuilding crate to form complex queries
        .filter(user.age.greater_than(Param::Number(10)))
        .and(user.name.equals(Param::String("John")))
        .build();


    // Then, it automatically builds the query params, i.e
    //
    //
    greater_than can return $_______randon_string_JJAJAJJA____(10)
    greater_than can return $_______randon_string_JJAJAJJA____("John")

    // Then, we get:  // SELECT * FROM User WHERE age > 10 AND name = 'John'
     // SELECT * FROM User WHERE age > randon_string_JJAJAJJA____(10) AND name = randon_string_JJAJAJJA____(10)
     // Automatically generates
     surreal::query(sql!(select with random _strings_raplaced with integers e.g age > $1 AND name = $2)).bind(("1", 10 )).bind(("2", "John"))

     // Alternatively, automatically extract the field name from e.g user.age.greater_than(..), by
     // displaying user.age and replace ., -> with underscore
     // DbField.greater_than(value)(self.to_string().repalce(".", "_").replace("->", "_"),replace(" ", "_"))
}
