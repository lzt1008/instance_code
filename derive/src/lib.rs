use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Ident, Path, PathSegment, Token,
};
use synstructure::{AddBounds, Structure};

#[proc_macro_derive(InstanceCode, attributes(instance))]
pub fn instance_code_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(instance_code_derive_impl(&input))
}

fn instance_code_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let mut structure = Structure::new(input);

    let path = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("instance"))
        .map(|a| a.parse_args::<PathAttr>().map(|p| p.0))
        .transpose()
        .expect("failed to parse path attribute");

    let body = structure.each_variant(|vi| {
        let quote_bindings = vi.bindings().iter().map(|b| {
            let ident = b.binding.clone();
            quote! { let #ident = #ident.instance_code(); }
        });

        let constructor = vi.construct(|_, i| {
            let ident = &vi.bindings()[i].binding;
            quote! { ##ident }
        });

        let constructor = match &path {
            Some(path) => quote! { #path::#constructor },
            None => constructor,
        };

        quote! {
            #(#quote_bindings)*
            instance_code::quote! { #constructor }
        }
    });

    structure.add_bounds(AddBounds::Fields);

    structure.gen_impl(quote! {
        gen impl instance_code::InstanceCode for @Self {
            fn instance_code(&self) -> instance_code::TokenStream {
                match self {
                    #body
                }
            }
        }
    })
}

struct PathAttr(Punctuated<PathSegment, Token![::]>);

impl Parse for PathAttr {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let i: Ident = input.parse()?;
        if i != "path" {
            return Err(input.error(format!("expected token \"path\", found {i:?}")));
        }
        input.parse::<Token![=]>()?;
        Ok(Self(input.parse::<Path>()?.segments))
    }
}
