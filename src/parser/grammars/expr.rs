use super::operations::pipe;
use crate::{common::token::Token, parser::node_match::NodeMatch};

#[inline(always)]
pub fn expr<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<NodeMatch<'a>> {
    pipe(tokens, offset)
}
