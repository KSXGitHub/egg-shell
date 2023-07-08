mod function;
mod module;
mod variable;

pub use function::*;
pub use module::*;
pub use variable::*;

#[derive(Debug)]
pub enum Declaration {
    Module(ModuleDeclaration),
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}
