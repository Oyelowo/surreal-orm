pub struct FieldGenericsMeta<'a> {
    pub(crate) field_impl_generics: syn::ImplGenerics<'a>,
    pub(crate) field_ty_generics: syn::TypeGenerics<'a>,
    pub(crate) field_where_clause: Option<&'a syn::WhereClause>,
}

impl<'a> FieldGenericsMeta<'a> {
    // This extracts generics metadata for field and from struct generics metadata.
    // This could come from the concrete rust field type or
    // as an attribute on the field from links which link to
    // other tables structs models i.e Edge, Node and Objects.
    // These are usually specified using the link_one, link_self
    // and link_many and relate attributes.
    // e.g
    // #[surreal_orm(link_one = User<'a, T, u32>)]
    // student: LinkOne<User<'a, T, u32>
    pub fn new(
        &self,
        struct_name_ident: &Ident,
        struct_generics: &Generics,
        field_type: &Type,
    ) -> FieldGenericsMeta<'a> {
        let (_, struct_ty_generics, _) = struct_generics.split_for_impl();
        let field_type =
            &replace_self_in_type_str(&field_type, struct_name_ident, &struct_ty_generics);
        let mut field_extractor = GenericTypeExtractor::new(struct_generics);
        let (field_impl_generics, field_ty_generics, field_where_clause) = field_extractor
            .extract_generics_for_complex_type(&field_type)
            .split_for_impl();
        FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        }
    }
}
