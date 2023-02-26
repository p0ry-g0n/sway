use crate::priv_prelude::*;

#[derive(Clone, Debug)]
pub enum AsmBlock {
    Fuel(FuelAsmBlock),
    Miden(MidenAsmBlock),
}

#[derive(Clone, Debug)]
pub struct MidenAsmBlock {
    pub asm_token: AsmToken,
    pub contents: Braces<MidenAsmBlockContents>,
}

#[derive(Clone, Debug)]
pub struct MidenAsmBlockContents {
    pub instructions: Vec<(Instruction, SemicolonToken)>,
    pub final_expr_opt: Option<AsmFinalExpr>,
}

#[derive(Clone, Debug)]
pub struct FuelAsmBlock {
    pub asm_token: AsmToken,
    pub registers: Parens<Punctuated<AsmRegisterDeclaration, CommaToken>>,
    pub contents: Braces<AsmBlockContents>,
}

#[derive(Clone, Debug)]
pub struct AsmRegisterDeclaration {
    pub register: Ident,
    pub value_opt: Option<(ColonToken, Box<Expr>)>,
}

#[derive(Clone, Debug)]
pub struct AsmBlockContents {
    pub instructions: Vec<(Instruction, SemicolonToken)>,
    pub final_expr_opt: Option<AsmFinalExpr>,
}

#[derive(Clone, Debug)]
pub struct AsmFinalExpr {
    pub register: Ident,
    pub ty_opt: Option<(ColonToken, Ty)>,
}

#[derive(Clone, Debug)]
pub struct AsmImmediate {
    pub span: Span,
    pub parsed: BigUint,
}

impl Spanned for AsmImmediate {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for FuelAsmBlock {
    fn span(&self) -> Span {
        Span::join(self.asm_token.span(), self.contents.span())
    }
}

impl Spanned for AsmBlock {
    fn span(&self) -> Span {
        match self {
            AsmBlock::Fuel(asm_block) => asm_block.span(),
            AsmBlock::Miden(_) => todo!(),
        }
    }
}
