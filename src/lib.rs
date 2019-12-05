extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

macro_rules! err {
    ($meta: expr) => {
        err!($meta, r#"expected `def = "..."`"#)
    };
    ($meta: expr, $err: expr) => {
        syn::Error::new_spanned($meta, $err).to_compile_error()
    };
}

#[proc_macro_derive(Defaults, attributes(def))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        fields
    } else {
        return err!(name, "Only implemented for structs").into();
    };

    match fields {
        syn::Fields::Named(fields) => named_impl(name, fields),
        syn::Fields::Unnamed(fields) => unnamed_impl(name, fields),
        syn::Fields::Unit => unit_impl(name),
    }
}

fn def_of(f: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &f.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "def" {
            return Some(attr);
        }
    }
    None
}

fn def_val_of(attr: &syn::Attribute) -> proc_macro2::TokenStream {
    let nv = match attr.parse_meta() {
        Ok(syn::Meta::NameValue(nv)) => nv,
        Ok(meta) => return err!(meta),
        Err(e) => return e.to_compile_error(),
    };

    match &nv.lit {
        syn::Lit::Str(s) => match s.parse() {
            Ok(ok) => ok,
            Err(e) => e.to_compile_error(),
        },
        _ => err!(nv),
    }
}

fn unit_impl(name: &proc_macro2::Ident) -> TokenStream {
    let val = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self
            }
        }
    };
    val.into()
}

fn named_impl(name: &proc_macro2::Ident, fields: &syn::FieldsNamed) -> TokenStream {
    let fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let span = name.as_ref().unwrap().span();
        if let Some(attr) = def_of(f) {
            let def_val = def_val_of(attr);
            quote_spanned! {span => #name: #def_val }
        } else {
            quote_spanned! {span => #name: Default::default() }
        }
    });

    let out = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self {
                    #(#fields,)*
                }
            }
        }
    };
    out.into()
}

fn unnamed_impl(name: &proc_macro2::Ident, fields: &syn::FieldsUnnamed) -> TokenStream {
    use syn::spanned::Spanned;
    let fields = fields.unnamed.iter().map(|f| {
        if let Some(attr) = def_of(f) {
            let def_val = def_val_of(attr);
            quote_spanned! {f.span() => #def_val }
        } else {
            quote_spanned! {f.span() => Default::default() }
        }
    });

    let out = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self (
                    #(#fields,)*
                )
            }
        }
    };
    out.into()
}
