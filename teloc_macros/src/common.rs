use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItem, ImplItemMethod, ItemImpl};

pub fn compile_error<T: ToTokens>(data: T) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#data);
    }
}

pub fn get_1_teloc_attr(attrs: &[Attribute]) -> Result<Option<&Attribute>, TokenStream> {
    let mut teloc_attrs = vec![];
    attrs.iter().for_each(|attr| {
        if attr.path.is_ident("init") {
            teloc_attrs.push(attr);
        }
    });
    match teloc_attrs.as_slice() {
        [] => Ok(None),
        [x] => Ok(Some(x)),
        _ => Err(compile_error(format!(
            "Expected 0 or 1 `clone` or `init` attribute, found {}",
            teloc_attrs.len()
        ))),
    }
}

pub fn get_1_method(item: ItemImpl) -> Result<ImplItemMethod, syn::Error> {
    let span = item.span();
    let methods = item
        .items
        .into_iter()
        .filter_map(|x| match x {
            ImplItem::Method(method) => Some(method),
            _ => None,
        })
        .collect::<Vec<_>>();
    match methods.as_slice() {
        [_] => Ok(methods.into_iter().next().unwrap()),
        _ => Err(syn::Error::new(span, "Expected one method in impl!")),
    }
}

pub fn ident_generator(count: usize) -> Vec<Ident> {
    name_generator()
        .take(count)
        .map(|s| Ident::new(&s, Span::call_site()))
        .collect()
}

pub fn name_generator() -> impl Iterator<Item = String> {
    const ALPHABET: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    (1..)
        .map(|i| {
            ALPHABET
                .iter()
                .combinations_with_replacement(i)
                .map(|arr| arr.iter().join(""))
        })
        .flatten()
}
