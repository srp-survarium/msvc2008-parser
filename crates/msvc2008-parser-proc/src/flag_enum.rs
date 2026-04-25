use quote::quote;
use syn::{LitInt, LitStr, Token};

use crate::bail;

struct FlagEntries {
    name: syn::Ident,
    entries: Vec<FlagEntry>,
}

struct FlagEntry {
    value: i8,
    flag: String,
}

impl syn::parse::Parse for FlagEntries {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![enum]>()?;
        let name = input.parse::<syn::Ident>()?;

        let content;
        syn::braced!(content in input);

        let mut entries = vec![];
        while !content.is_empty() {
            let n: LitInt = content.parse()?;
            content.parse::<Token![=>]>()?;

            let flag: LitStr = content.parse()?;
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }

            entries.push(FlagEntry {
                value: n.base10_parse::<i8>()?,
                flag: flag.value(),
            });
        }

        Ok(Self { name, entries })
    }
}

pub fn flag_enum(input: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let FlagEntries { name, entries } = syn::parse::<FlagEntries>(input)?;

    let name_str = name.to_string();

    if entries.is_empty() {
        bail!(name, "flag_enum requires at least one entry");
    }

    let parse_arms: proc_macro2::TokenStream = entries
        .iter()
        .map(|FlagEntry { value, flag }| {
            quote! { #value => Ok(Self(#flag)), }
        })
        .collect();

    Ok(quote! {
        #[repr(transparent)]
        struct #name(&'static str);

        impl #name {
            fn parse(v: i8) -> anyhow::Result<Self> {
                match v {
                    #parse_arms
                    _ => anyhow::bail!("Invalid {} value: {}", #name_str, v),
                }
            }

            fn as_str(&self) -> &'static str {
                self.0
            }
        }

        impl std::str::FromStr for #name {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> anyhow::Result<Self> {
                Self::parse(s.parse()?)
            }
        }
    })
}
