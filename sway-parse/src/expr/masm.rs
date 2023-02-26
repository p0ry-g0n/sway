use crate::expr::op_code::parse_instruction;
use crate::{Parse, ParseResult, ParseToEnd, Parser, ParserConsumed};

use core::str::FromStr;
use num_bigint::BigUint;

use sway_ast::expr::asm::{
    AsmBlock, AsmBlockContents, AsmFinalExpr, AsmImmediate, AsmRegisterDeclaration, MidenAsmBlock,
};
use sway_error::parser_error::ParseErrorKind;
use sway_types::{Ident, Spanned};

impl Parse for MidenAsmBlock {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        todo!()
    }
}
