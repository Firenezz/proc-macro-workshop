use syn::{GenericArgument, PathArguments, Type};



pub fn get_non_optional_type(ty: &Type) -> &syn::Type {
    if let Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.first() {
            if seg.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                        ty
                    } else {
                        panic!("This field should have a type path")
                    }
                } else {
                    panic!("This field should have an angle bracketed path")
                }
            } else {
                panic!("This field should have Option as the first segment")
            }
        } else {
            panic!("This field should have a type path")
        }
    } else {
        unimplemented!("Unsupported type")
    }
}