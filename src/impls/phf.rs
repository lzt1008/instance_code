use proc_macro2::TokenStream;
use quote::quote;

use crate::InstanceCode;

impl<K, V> InstanceCode for phf::Map<K, V>
where
    K: InstanceCode,
    V: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let data = self
            .entries()
            .map(|(k, v)| {
                let k = k.instance_code();
                let v = v.instance_code();
                quote!(#k => #v)
            })
            .collect::<Vec<_>>();
        quote! {
            phf::phf_map! { #(#data),* }
        }
    }
}
