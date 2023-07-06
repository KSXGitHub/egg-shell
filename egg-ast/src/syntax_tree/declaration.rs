mod module;
mod variable;

pub use module::*;
pub use variable::*;

#[derive(Debug)]
pub enum Declaration {
    Module(ModuleDeclaration),
    Variable(VariableDeclaration),
}
