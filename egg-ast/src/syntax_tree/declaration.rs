mod constant;
mod function;
mod r#macro;
mod module;
mod variable;

pub use constant::*;
pub use function::*;
pub use module::*;
pub use r#macro::*;
pub use variable::*;

#[derive(Debug)]
pub enum Declaration {
    Constant(ConstantDeclaration),
    Function(FunctionDeclaration),
    Module(ModuleDeclaration),
    Macro(MacroDeclaration),
    Variable(VariableDeclaration),
}
