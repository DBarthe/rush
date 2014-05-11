use collections::HashMap;
use shell::BuiltinFn;

pub mod exit;
pub mod eval;

pub fn create_builtins() -> Box<HashMap<~str, BuiltinFn>>
{
    let mut builtins: HashMap<~str, BuiltinFn> = HashMap::new();

    builtins.insert(exit::NAME.to_owned(), exit::builtin);
    builtins.insert(eval::NAME.to_owned(), eval::builtin);
    
    box builtins
}
