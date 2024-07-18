use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, Data};

pub(crate) struct BuilderField {
    inner_field: syn::Field,
    name: syn::Ident,
    ty: syn::Type,
}

impl BuilderField {
    fn new(field: syn::Field) -> Self {
        let field = syn::Field::from(field);
        let name = field.ident.clone().unwrap();
        let ty = field.ty.clone();
        BuilderField {
            inner_field: field,
            name,
            ty,
        }
    }

    fn to_tokens_with_optional_type(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        quote_spanned! {
            name.span() =>
            #name: Option<#ty>
        }
    }

    fn default_optional_field_tokens(&self) -> TokenStream {
        let name = &self.name;
        quote_spanned! {
            name.span() =>
            #name: None
        }
    }
}

pub struct BuilderFields {
    fields: Vec<BuilderField>,
}

impl BuilderFields {
    fn new(fields: Vec<syn::Field>) -> Self {
        BuilderFields {
            fields: fields.into_iter().map(BuilderField::new).collect(),
        }
    }
}

impl From<Data> for BuilderFields {
    fn from(data: Data) -> Self {
        BuilderFields::new(extract_fields(&data))
    }
}

pub struct BuilderEmitter {
    name: syn::Ident,
    builder_name: syn::Ident,
    fields: BuilderFields,
}

impl BuilderEmitter {
    fn new(name: syn::Ident, fields: BuilderFields) -> Self {
        let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());
        BuilderEmitter {
            name,
            builder_name,
            fields,
        }
    }

    pub fn init_builder_tokens(&self) -> TokenStream {
        let fields = &self.fields.fields;
        let recurse = fields
            .iter()
            .map(|field| field.default_optional_field_tokens());
        let builder_name = &self.builder_name;
        let name = &self.name;
        quote::quote! {
            impl #name {
                pub fn builder() -> #builder_name {
                    #builder_name {
                        #(#recurse),*
                    }
                }
            }
        }
    }

    pub fn builder_definition_tokens(&self) -> TokenStream {
        let fields = &self.fields.fields;
        let recurse = fields
            .iter()
            .map(|field| field.to_tokens_with_optional_type());
        let builder_name = &self.builder_name;
        quote::quote! {
            pub struct #builder_name {
                #( #recurse ),*
            }
        }
    }

    pub fn builder_emit_tokens(&self) -> TokenStream {
        let builder_fn = &self.init_builder_tokens();
        let builder_struct = &self.builder_definition_tokens();
        quote::quote! {
            #builder_struct

            #builder_fn
        }
    }
}

impl From<syn::DeriveInput> for BuilderEmitter {
    fn from(value: syn::DeriveInput) -> Self {
        BuilderEmitter::new(value.ident, value.data.into())
    }
}

fn extract_fields(data: &Data) -> Vec<syn::Field> {
    match *data {
        Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields.named.clone().into_iter().collect(),
            syn::Fields::Unnamed(ref fields) => fields.unnamed.clone().into_iter().collect(),
            syn::Fields::Unit => unimplemented!("Unit structs aren't supported yet."),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!("Enums and unions aren't supported yet."),
    }
}
