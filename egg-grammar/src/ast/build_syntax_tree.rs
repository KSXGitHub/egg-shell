#[derive(Debug)]
pub struct BuildSyntaxTree<TokenIter> {
    source: TokenIter,
}

impl<TokenIter> BuildSyntaxTree<TokenIter> {
    pub fn from_token_iter(source: TokenIter) -> Self {
        BuildSyntaxTree { source }
    }
}
