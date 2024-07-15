
    pub fn struct_no_bounds(&self) -> ExtractorResult<CustomTypeNoSelf> {
        // let replacement_path: Path = parse_quote!(#struct_name #ty_generics);
        self.construct_struct_type_without_bounds()
            .replace_self_with_current_struct_concrete_type(self)
    }

    fn construct_struct_type_without_bounds(&self) -> CustomType {
        let mut path = Path::from(self.ident());
        let generics = self.generics().to_basic_generics_ref();

        // Process generics, excluding bounds
        if !generics.params.is_empty() {
            let args = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Type(type_param) => {
                        GenericArgument::Type(parse_quote!(#type_param))
                    }
                    syn::GenericParam::Lifetime(lifetime_def) => {
                        GenericArgument::Lifetime(lifetime_def.lifetime.clone())
                    }
                    syn::GenericParam::Const(const_param) => {
                        // TODO: Test this in struct
                        GenericArgument::Const(
                            const_param
                                .default
                                .clone()
                                .expect("absent const expression"),
                        )
                    }
                })
                .collect();

            path.segments
                .last_mut()
                .expect("Problem getting last segment of path. Path potentially empty.")
                .arguments = PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: generics.lt_token.expect("Missing lt token"),
                args,
                gt_token: generics.gt_token.expect("Missing gt token"),
            });
        }

        Type::Path(syn::TypePath { qself: None, path }).into()
    }
}
