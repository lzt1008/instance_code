use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::BuildHasher,
    rc::Rc,
    sync::Arc,
};

use super::*;

impl InstanceCode for String {
    fn instance_code(&self) -> TokenStream {
        quote! {
            #self.to_owned()
        }
    }
}

impl<T: ?Sized + ToOwned> InstanceCode for Cow<'_, T>
where
    for<'a> &'a T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let t = <&T as InstanceCode>::instance_code(&&**self);
        quote! {
            Cow::Borrowed(#t)
        }
    }
}

impl<T> InstanceCode for Vec<T>
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let data = self.iter().map(InstanceCode::instance_code);
        quote! {
            vec![#(#data,)*]
        }
    }
}

impl<T> InstanceCode for BTreeSet<T>
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let data = self.iter().map(InstanceCode::instance_code);
        quote! {
            std::collections::BTreeSet::from([#(#data),*])
        }
    }
}

impl<K, V> InstanceCode for BTreeMap<K, V>
where
    K: InstanceCode,
    V: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let data = self.iter().map(|(k, v)| {
            let k = k.instance_code();
            let v = v.instance_code();
            quote!((#k, #v))
        });
        quote! {
            std::collections::BTreeMap::from([#(#data),*])
        }
    }
}

impl<T, S> InstanceCode for HashSet<T, S>
where
    T: InstanceCode,
    S: BuildHasher + Default,
{
    fn instance_code(&self) -> TokenStream {
        let mut data = self
            .iter()
            .map(InstanceCode::instance_code)
            .collect::<Vec<_>>();
        data.sort_unstable_by_key(|data| data.to_string());
        quote! {
            std::collections::HashSet::from_iter([#(#data),*])
        }
    }
}

impl<K, V, S> InstanceCode for HashMap<K, V, S>
where
    K: InstanceCode,
    V: InstanceCode,
    S: BuildHasher + Default,
{
    fn instance_code(&self) -> TokenStream {
        let mut data = self
            .iter()
            .map(|(k, v)| {
                let k = k.instance_code();
                let v = v.instance_code();
                quote!((#k, #v))
            })
            .collect::<Vec<_>>();
        data.sort_unstable_by_key(|data| data.to_string());
        quote! {
            std::collections::HashMap::from_iter([#(#data),*])
        }
    }
}

macro_rules! smart_pointer {
    ($type:ty, $constructor:path) => {
        impl<T> InstanceCode for $type
        where
            T: InstanceCode,
        {
            fn instance_code(&self) -> TokenStream {
                let data = std::ops::Deref::deref(self).instance_code();
                quote! {
                    $constructor(#data)
                }
            }
        }
    };
}

smart_pointer!(Box<T>, std::boxed::Box::new);
smart_pointer!(Rc<T>, std::rc::Rc::new);
smart_pointer!(Arc<T>, std::sync::Rc::new);
