extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_uniform_func(_: TokenStream) -> TokenStream {
    let mut result = String::new();
    let type_list = ["f", "d", "i", "ui"];
    let rust_type_list = HashMap::from([
        ("f", "f32"),
        ("d", "f64"),
        ("i", "i32"),
        ("ui", "u32"),
    ]);
    for dimension in 1..=4 {
        for ty in type_list {
            let mut func = format!("pub fn uniform{}{}(location: i32", dimension, ty);
            let (arg_list, arg_name) = {
                let mut arg_list = String::new();
                let mut arg_name = String::new();
                for arg in 1..=dimension {
                    arg_list.push_str(&format!(" ,component{}: {}", arg, rust_type_list[ty]));
                    arg_name.push_str(&format!(",component{}", arg));
                }
                (arg_list, arg_name)
            };
            func.push_str(&arg_list);
            func.push_str(&format!(") {{
                unsafe {{
                    gl::Uniform{}{}(location ", dimension, ty));
            func.push_str(&arg_name);
            func.push_str(")
                }
            }");
            result.push_str(&func);
            result.push_str("\n");
        }
    }
    result.parse().unwrap()
}
