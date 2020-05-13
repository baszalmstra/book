# extern crate mun_runtime;
use mun_runtime::{invoke_fn, RuntimeBuilder, StructRef};
use std::{cell::RefCell, env, rc::Rc};

fn main() {
    let lib_path = env::args().nth(1).expect("Expected path to a Mun library.");

    let mut runtime = Rc::new(RefCell::new(
        RuntimeBuilder::new(lib_path)
            .spawn()
            .expect("Failed to spawn Runtime"),
    ));

    let a: StructRef = invoke_fn!(runtime, "vector2_new", -1.0f32, 1.0f32).unwrap();
    let b: StructRef = invoke_fn!(runtime, "vector2_new", 1.0f32, -1.0f32).unwrap();
    let added: StructRef = invoke_fn!(runtime, "vector2_add", a, b).unwrap();
}
