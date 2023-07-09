mod constant;
mod function;
mod module;
mod variable;

pub use constant::*;
pub use function::*;
pub use module::*;
pub use variable::*;

#[derive(Debug)]
pub enum Declaration {
    Constant(ConstantDeclaration),
    Function(FunctionDeclaration),
    Module(ModuleDeclaration),
    Variable(VariableDeclaration),
}
