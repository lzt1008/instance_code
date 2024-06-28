use std::marker::PhantomData;

use super::*;

macro_rules! primitive {
    ($($type:ty),*) => {
        $(
            impl InstanceCode for $type {
                fn instance_code(&self) -> TokenStream {
                    quote! {
                        #self
                    }
                }
            }
        )*
    }
}

primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, &str, char, bool
);

impl<'a, T: ?Sized> InstanceCode for &'a T
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let t = <T as InstanceCode>::instance_code(*self);
        quote! {
            &#t
        }
    }
}

impl<T> InstanceCode for [T]
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        let slice = self.iter().map(|d| d.instance_code());
        quote!([#(#slice),*])
    }
}

impl<T, const N: usize> InstanceCode for [T; N]
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        self.as_slice().instance_code()
    }
}

impl<T> InstanceCode for Option<T>
where
    T: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        match self {
            None => quote! { None },
            Some(t) => {
                let t = t.instance_code();
                quote! { Some(#t) }
            }
        }
    }
}

impl<T, E> InstanceCode for Result<T, E>
where
    T: InstanceCode,
    E: InstanceCode,
{
    fn instance_code(&self) -> TokenStream {
        match self {
            Ok(ok) => {
                let ok = ok.instance_code();
                quote!(Ok(#ok))
            }
            Err(e) => {
                let e = e.instance_code();
                quote!(Err(#e))
            }
        }
    }
}

impl<T> InstanceCode for PhantomData<T> {
    fn instance_code(&self) -> TokenStream {
        quote! {
            ::core::marker::PhantomData
        }
    }
}

macro_rules! tuple {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($ty),*> InstanceCode for ($($ty,)*) where $($ty: InstanceCode),* {
            fn instance_code(&self) -> TokenStream {
                let ($($ty,)*) = self;
                $(
                    let $ty = $ty.instance_code();
                )*
                quote! {
                    ($(#$ty),*)
                }
            }
        }
    }
}

tuple!();
tuple!(A);
tuple!(A, B);
tuple!(A, B, C);
tuple!(A, B, C, D);
tuple!(A, B, C, D, E);
tuple!(A, B, C, D, E, F);
tuple!(A, B, C, D, E, F, G);
tuple!(A, B, C, D, E, F, G, H);
tuple!(A, B, C, D, E, F, G, H, I);
tuple!(A, B, C, D, E, F, G, H, I, J);
