use file::parse;
use serde::de::DeserializeOwned;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::InstanceCode;

pub mod file;

pub fn provide_instance_code(key: &str, code: &str) {
    let path =
        Path::new(&env::var("OUT_DIR").unwrap()).join(format!("__{}__instance_codegen.rs", key));
    let mut file = BufWriter::new(File::create(path).unwrap());

    file.write_all(code.as_bytes()).unwrap();
}

pub fn provide_instance<T>(key: &str, file: &str)
where
    T: DeserializeOwned + InstanceCode,
{
    let instance: T = parse(file).unwrap_or_else(|e| panic!("{e}"));

    println!("cargo:rerun-if-changed={}", file);

    #[cfg(feature = "pretty")]
    {
        let file = format!(
            " const _: () = {{ {} }};",
            instance.instance_code().to_string()
        );
        let formatted = prettyplease::unparse(&syn::parse_str(&file).unwrap());

        provide_instance_code(
            key,
            &formatted
                .trim_end_matches("};\n")
                .trim_start_matches("const _: () = {"),
        );
    }

    #[cfg(not(feature = "pretty"))]
    provide_instance_code(key, &instance.instance_code().to_string());
}
