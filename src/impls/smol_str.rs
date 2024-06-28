use proc_macro2::TokenStream;
use quote::quote;

use crate::InstanceCode;

impl InstanceCode for smol_str::SmolStr {
    fn instance_code(&self) -> TokenStream {
        let data = self.as_str();
        quote! {
            smol_str::SmolStr::from(#data)
        }
    }
}
