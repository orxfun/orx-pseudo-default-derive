use alloc::vec::Vec;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput, Fields, Ident};

pub(crate) fn derive_pseudo_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_pseudo_default(ast)
}

fn impl_pseudo_default(ast: DeriveInput) -> TokenStream {
    let typ = ast.ident;

    match ast.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Unit | Fields::Named(_) => impl_for_named_struct(typ, data),
            Fields::Unnamed(_) => impl_for_unnamed_struct(typ, data),
        },
        syn::Data::Enum(data) => impl_for_enum(typ, data),
        syn::Data::Union(data) => impl_for_union(typ, data),
    }
}

fn impl_for_named_struct(typ: Ident, data: DataStruct) -> TokenStream {
    let fields: Vec<_> = data.fields.into_iter().filter_map(|f| f.ident).collect();
    quote! {
        impl PseudoDefault for #typ {
            fn pseudo_default() -> Self {
                Self {
                    #(#fields: PseudoDefault::pseudo_default()), *
                }
            }
        }
    }
    .into()
}

fn impl_for_unnamed_struct(typ: Ident, data: DataStruct) -> TokenStream {
    let fields: Vec<_> = data
        .fields
        .into_iter()
        .map(|_| quote! { PseudoDefault::pseudo_default() })
        .collect();

    let arguments = quote! { #(#fields, )* };

    quote! {
        impl PseudoDefault for #typ {
            fn pseudo_default() -> Self {
                Self(#arguments)
            }
        }
    }
    .into()
}

fn impl_for_enum(_typ: Ident, _data: DataEnum) -> TokenStream {
    todo!("Deriving PseudoDefault for enums is not supported yet.")
}

fn impl_for_union(_typ: Ident, _data: DataUnion) -> TokenStream {
    todo!("Deriving PseudoDefault for unions is not supported yet.")
}
