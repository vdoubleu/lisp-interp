pub fn clean(prog: &String) -> String {
    return prog.trim().replace("\n", " ").replace("\t", " ");
}
