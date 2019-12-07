extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned;

macro_rules! err {
    ($meta: expr) => {
        err!($meta, r#"expected `def = "..."`"#)
    };
    ($meta: expr, $err: expr) => {
        Err(syn::Error::new_spanned($meta, $err))
    };
}

#[proc_macro_derive(Defaults, attributes(def))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(s) => derive_struct(name, &s.fields),
        _ => syn::Error::new_spanned(name, "`Defaults` cannot be derived for enums, only structs")
            .to_compile_error()
            .into(),
    }
}

fn derive_struct(name: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let tts = match fields {
        syn::Fields::Named(fields) => struct_named_impl(name, fields),
        syn::Fields::Unnamed(fields) => struct_unnamed_impl(name, fields),
        syn::Fields::Unit => struct_unit_impl(name),
    };

    match tts {
        Ok(tts) => tts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn def_attr(f: &syn::Field) -> syn::Result<Option<&syn::Attribute>> {
    let mut out = None;
    for attr in &f.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "def" {
            if out.is_none() {
                out = Some(attr);
            } else {
                err!(attr, r#"multiple definitions of "def" found"#)?;
            }
        }
    }
    Ok(out)
}

fn def_val_of(attr: &syn::Attribute) -> syn::Result<proc_macro2::TokenStream> {
    let nv = match attr.parse_meta()? {
        syn::Meta::NameValue(nv) => nv,
        meta => err!(meta)?,
    };

    match &nv.lit {
        syn::Lit::Str(s) => match s.parse::<syn::Expr>() {
            Ok(expr) => Ok(quote! { #expr }),
            _ => err!(s, "Not a valid expression"),
        },
        _ => err!(nv),
    }
}

fn struct_unit_impl(name: &proc_macro2::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let val = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self
            }
        }
    };
    Ok(val)
}

fn struct_named_impl(
    name: &proc_macro2::Ident,
    fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut mapped = vec![];
    for f in &fields.named {
        let name = &f.ident;
        let mapped_field = match def_attr(f)? {
            Some(attr) => {
                let def_val = def_val_of(attr)?;
                quote_spanned! {f.span() => #name: #def_val }
            }
            None => quote_spanned! {f.span() => #name: Default::default() },
        };
        mapped.push(mapped_field);
    }

    let out = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self {
                    #(#mapped,)*
                }
            }
        }
    };
    Ok(out)
}

fn struct_unnamed_impl(
    name: &proc_macro2::Ident,
    fields: &syn::FieldsUnnamed,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut mapped = vec![];
    for f in &fields.unnamed {
        let f = match def_attr(f)? {
            Some(attr) => {
                let def_val = def_val_of(attr)?;
                quote_spanned! {f.span() => #def_val }
            }
            None => quote_spanned! {f.span() => Default::default() },
        };
        mapped.push(f);
    }

    let out = quote! {
        impl std::default::Default for #name {
            fn default() -> Self {
                Self (
                    #(#mapped,)*
                )
            }
        }
    };
    Ok(out)
}
