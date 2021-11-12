extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(Hydrate)]
pub fn hydrate_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    hydrate_macro(ast)
}

fn is_state_type(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => match segments.first() {
            Some(segment) => segment.ident == "State",
            None => false,
        },
        _ => false,
    }
}

fn hydrate_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;
    let non_state_fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(FieldsNamed { named, .. }) => named
                .iter()
                .filter_map(|f| {
                    if is_state_type(&f.ty) {
                        None
                    } else {
                        Some(&f.ident)
                    }
                })
                .collect(),
            Fields::Unit => vec![],
            _ => panic!(),
        },
        _ => panic!("expected a struct"),
    };
    let state_fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                named
                    .iter()
                    .filter_map(|f| {
                        // panic!("Type: {:?}", f.ty);
                        if is_state_type(&f.ty) {
                            Some(&f.ident)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            Fields::Unit => vec![],
            _ => panic!(),
        },
        _ => panic!("expected a struct"),
    };
    let is_dirty_method = if state_fields.is_empty() {
        quote! {}
    } else {
        quote! {
            fn is_dirty(&self) -> bool {
                #(self.#state_fields.is_dirty()) || *
            }
        }
    };
    let gen = quote! {
        impl Hydrate for #name {
            fn hydrate(&mut self, other: &Self) {
                #(self.#non_state_fields.clone_from(&other.#non_state_fields);) *
            }
            fn is_same(&self, other: &Self) -> bool {
                self.eq(&other)
            }
            #is_dirty_method
        }
    };
    gen.into()
}
