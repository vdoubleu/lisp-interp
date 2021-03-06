use crate::runner::run_prog_with_store;
use crate::reader::read;
use crate::ast_type::{
    Res,
};
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn import_module(path: &String, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    let mut file_interp_args: InterpArgs = interp_args.clone();

    let file_path = if let Some(p) = find_stdlib(path, interp_args) {
        p
    } else {
        path.clone()
    };

    file_interp_args.file_name = Some(file_path);
    let file_contents = read(&file_interp_args);
    
    let res = run_prog_with_store(&file_contents, store, interp_args);
    return res;
}

fn find_stdlib(lib_name: &String, interp_args: &InterpArgs) -> Option<String> {
    let lib_path = if interp_args.lib_path != "" { 
        format!{"{}/{}.fl", interp_args.lib_path, lib_name} 
    } else { 
        format!{"src/stdlib/{}.fl", lib_name} 
    };

    let include_path = format!("/usr/include/fl/{}.fl", lib_name);

    if std::path::Path::new(&lib_path).exists() {
        return Some(lib_path);
    } else if std::path::Path::new(&include_path).exists() {
        return Some(include_path);
    } else {
        return None;
    }
}

pub fn get_stdlib(def: &String) -> Option<String> {
    let list_defs = HashSet::from(["second", "third", "last", "reverse", "empty?", "ind", "append"]);
    let string_defs = HashSet::from(["concat-string-list", "substr-start", "substr-end"]);

    if list_defs.contains(&def.as_str()) {
        return Some(String::from("list"));
    } else if string_defs.contains(&def.as_str()) {
        return Some(String::from("string"));
    } else {
        return None;
    }
}