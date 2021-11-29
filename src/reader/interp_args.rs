#[derive(Debug, Clone)]
pub struct InterpArgs {
    pub no_interp: bool,
    pub print_tokens: bool,
    pub file_name: Option<String>
}

impl InterpArgs {
    pub fn default() -> InterpArgs {
        InterpArgs {
            no_interp: false,
            print_tokens: false,
            file_name: None
        }
    }
}