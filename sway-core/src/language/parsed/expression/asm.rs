use super::Expression;
use crate::{
    language::{AsmOp, AsmRegister},
    TypeInfo,
};
use sway_types::{ident::Ident, span::Span};

#[derive(Debug, Clone)]
pub struct AsmExpression {
    pub(crate) content: AsmContent,
    pub(crate) return_type: TypeInfo,
    pub(crate) whole_block_span: Span,
}

#[derive(Debug, Clone)]
pub enum AsmContent {
    Fuel(FuelAsmExpression),
}

#[derive(Debug, Clone)]
pub struct FuelAsmExpression {
    pub registers: Vec<AsmRegisterDeclaration>,
    pub(crate) body: Vec<AsmOp>,
    pub(crate) returns: Option<(AsmRegister, Span)>,
}
impl AsmExpression {
    pub(crate) fn target_arch(&self) -> crate::BuildTarget {
        match self.content {
            AsmContent::Fuel(..) => crate::BuildTarget::Fuel,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AsmRegisterDeclaration {
    pub(crate) name: Ident,
    pub initializer: Option<Expression>,
}
