use crate::common::ast::ASTNode;

#[derive(Debug)]
pub struct NodeMatch<'a> {
    pub ast: ASTNode<'a>,
    pub offset: usize,
}
