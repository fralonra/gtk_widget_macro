//! Documentation: https://docs.rs/gtk_widget_macro
//! crates.io: https://crates.io/crates/gtk_widget_macro
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(GtkWidget)]
pub fn gtk_widget_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_gtk_widget_macro(&ast)
}

fn impl_gtk_widget_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let idents = if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            fields_named
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let ty = match &f.ty {
                        Type::Path(type_path) => {
                            let segment = type_path.path.segments.first().unwrap();
                            &segment.ident
                        }
                        _ => panic!("Unknown type"),
                    };
                    (ident, ty)
                })
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let build_str = idents.iter().map(|(ident, ty)| {
        let expect_str = format!("Couldn't get {}", ident);
        let ident_str = ident.to_string();
        quote! {
            let #ident: #ty = builder.object(#ident_str).expect(#expect_str);
        }
    });
    let build_fields_str = idents.iter().map(|(ident, _ty)| {
        quote! {
            #ident,
        }
    });
    let get_fields_str = idents.iter().map(|(ident, ty)| {
        quote! {
            pub fn #ident(&self) -> &#ty {
                &self.#ident
            }
        }
    });

    let gen = quote! {
        impl #name {
            pub fn from_builder(builder: &Builder) -> Self {
                #(#build_str)*

                Self {
                    #(#build_fields_str)*
                }
            }

            #(#get_fields_str)*
        }
    };
    gen.into()
}
