/// object! macro is a syntactic sugar for array of setters (e.g`[age.equal_to(4), name.equal_to(param)]`)
/// for setting values in a `create` or `update` statement but provides
/// stricter checking  - to ensure that all fields are provided and
/// belong to the struct -  than using the basic struct to initialize values
/// Compared to using a normal struct, this also allows using `parameter` or `field` as value.
/// Example
/// ```rust, ignore
/// let space_ship1 = create::<SpaceShip>()
///     .set(object!(SpaceShip {
///            id: spaceship_id_1,
///            name: "SpaceShip1",
///            created: Utc::now(),
///        }))
///     .get_one(db.clone())
///     .await?;
/// ```
#[macro_export]
macro_rules! object {
    ($struct_name:ident { $($key:ident: $value:expr),* $(,)? }) => {
        {
            $crate::validators::assert_same_length_arrays($struct_name:: __get_serializable_field_names(), [ $( stringify!($key) ),*]);
            $crate::check_unique_idents!($($key), *);

            type __StructNameRenamedFields = <$struct_name as $crate::Model>::StructRenamedCreator;
            $crate::validators::assert_fields!(__StructNameRenamedFields : $( $key ),*);

            let schema = &$struct_name::schema();

            [
                $( schema.$key .equal_to($value) ),*
            ]
        }
    };
}

/// object_partial! macro is just like `object!` macro but allow omitting some fields and is
/// a syntactic sugar for array of setters (e.g`[age.equal_to(4), name.equal_to(param)]`) for
/// setting values in a `create` or `update` statement but provides
/// stricter checking  - to ensure that all fields are provided and
/// belong to the struct -  than using the basic struct to initialize values
/// Compared to using a normal struct, this also allows using `parameter` or `field` as value.
///
/// Example
/// ```rust, ignore
///     let updated = update::<Weapon>(id)
/// .set(object_partial!(Weapon { strength: 923u64 }))
/// .return_one(db.clone())
/// .await?;
///
///
/// ```
#[macro_export]
macro_rules! object_partial {
    ($struct_name:ident { $($key:ident: $value:expr),* $(,)? }) => {
        {
            $crate::check_unique_idents!($($key), *);
            // type __StructNameRenamedFields = <$struct_name as $crate::Model>::NonNullUpdater;
            type __StructNameRenamedFields = <$struct_name as $crate::Model>::StructRenamedCreator;
            $crate::validators::assert_fields!(__StructNameRenamedFields : $( $key ),*);

            let schema = &$struct_name::schema();

            [
                $( schema.$key .equal_to($value) ),*
            ]
        }
    };
}
