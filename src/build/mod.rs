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

    provide_instance_code(key, &instance.instance_code().to_string());
}
