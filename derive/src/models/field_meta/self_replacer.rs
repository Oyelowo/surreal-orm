/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// Function to recursively replace `Self` in the type
pub fn replace_self_in_type_str(
    ty: &Type,
    struct_name: &syn::Ident,
    ty_generics: &syn::TypeGenerics,
) -> Type {
    // Create the replacement type
    let replacement_path: Path = parse_quote!(#struct_name #ty_generics);

    // Helper function to replace 'Self' in a path segment
    fn replace_segment(segment: &mut PathSegment, replacement_path: &Path) {
        if segment.ident == "Self" {
            if let Some(first_segment) = replacement_path.segments.first() {
                *segment = first_segment.clone();
            }
        } else if let PathArguments::AngleBracketed(angle_args) = &mut segment.arguments {
            for arg in angle_args.args.iter_mut() {
                println!("rara1---arg: {}", arg.to_token_stream().to_string());
                if let GenericArgument::Type(t) = arg {
                    *t = replace_type(t, replacement_path);
                }
            }
        }
    }

    // Function to handle replacement within types
    fn replace_type(ty: &Type, replacement_path: &Path) -> Type {
        match ty {
            Type::Path(type_path) => {
                let mut new_type_path = type_path.clone();
                for segment in &mut new_type_path.path.segments {
                    replace_segment(segment, replacement_path);
                }
                Type::Path(new_type_path)
            }
            Type::Reference(type_reference) => {
                let elem = Box::new(replace_type(&type_reference.elem, replacement_path));
                Type::Reference(TypeReference {
                    and_token: type_reference.and_token,
                    lifetime: type_reference.lifetime.clone(),
                    mutability: type_reference.mutability,
                    elem,
                })
            }
            // Extend to handle other types like Tuple, Array, etc.
            _ => ty.clone(),
        }
    }

    replace_type(ty, &replacement_path)
}
