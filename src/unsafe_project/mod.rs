use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;
use syn::{spanned::Spanned, Generics, Item, Result, Type, Attribute, Meta, NestedMeta};

mod enums;
mod structs;

/// The annotation for pinned type.
const PIN: &str = "pin";

pub(super) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let span = span!(input);
    match syn::parse2(input) {
        Ok(Item::Struct(item)) => {
            ensure_not_packed(&item.attrs).and_then(|_| structs::parse(args, item))
        }
        Ok(Item::Enum(item)) => {
            ensure_not_packed(&item.attrs).and_then(|_| enums::parse(args, item))
        }
        _ => Err(error!(span, "may only be used on structs or enums")),
    }
    .unwrap_or_else(|e| e.to_compile_error())
}

fn ensure_not_packed(attrs: &[Attribute]) -> Result<()> {
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Meta::List(l) = meta {
                if l.ident == "repr" {
                    for repr in l.nested.iter() {
                        if let NestedMeta::Meta(Meta::Word(w)) = repr {
                            if w == "packed" {
                                return Err(error!(w, "unsafe_project may not be used on #[repr(packed)] types"))
                            }
                        }
                    }
                }
            }
        }
    }
    return Ok(())
}

/// Makes the generics of projected type from the reference of the original generics.
fn proj_generics(generics: &Generics) -> Generics {
    let mut generics = generics.clone();
    generics.params.insert(0, syn::parse_quote!('__a));
    generics
}

// =================================================================================================
// conditional Unpin implementation

#[derive(Default)]
struct ImplUnpin(
    Option<(
        // generics
        Generics,
        // span
        Span,
    )>,
);

impl ImplUnpin {
    /// Parses attribute arguments.
    fn new(args: TokenStream, generics: &Generics) -> Result<Self> {
        match &*args.to_string() {
            "" => Ok(Self::default()),
            "Unpin" => Ok(Self(Some((generics.clone(), args.span())))),
            _ => Err(error!(args, "an invalid argument was passed")),
        }
    }

    fn push(&mut self, ty: &Type) {
        if let Some((generics, _)) = &mut self.0 {
            generics
                .make_where_clause()
                .predicates
                .push(syn::parse_quote!(#ty: ::core::marker::Unpin));
        }
    }

    /// Creates `Unpin` implementation.
    fn build(self, ident: &Ident) -> TokenStream {
        self.0
            .map(|(generics, span)| {
                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                quote_spanned! { span =>
                    impl #impl_generics ::core::marker::Unpin for #ident #ty_generics #where_clause {}
                }
            })
            .unwrap_or_default()
    }
}
