mod alloc;
pub mod build;
mod impls;
mod primitives;

#[doc(no_inline)]
pub use proc_macro2::TokenStream;

#[doc(no_inline)]
pub use quote::quote;

#[cfg(feature = "derive")]
pub use instance_code_derive::InstanceCode;

pub trait InstanceCode {
    fn instance_code(&self) -> TokenStream;
}

#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! inject_instance {
    ($file:literal) => {
        include!(concat!(
            env!("OUT_DIR"),
            "/",
            "__",
            $file,
            "__instance_codegen.rs"
        ))
        .into()
    };
}

#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! inject_instance {
    ($file:literal) => {
        include!(concat!(
            env!("OUT_DIR"),
            r"\\",
            "__",
            $file,
            "__instance_codegen.rs"
        ))
        .into()
    };
}
