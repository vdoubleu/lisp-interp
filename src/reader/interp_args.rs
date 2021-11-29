#[derive(Debug, Clone)]
pub struct InterpArgs {
    pub no_interp: bool,
    pub print_tokens: bool,
    pub file_name: Option<String>
}
