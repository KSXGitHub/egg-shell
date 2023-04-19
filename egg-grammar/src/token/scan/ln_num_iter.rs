use egg_ast::LnNum;
use std::str::Lines;

/// Iterate over numbered lines.
#[derive(Debug)]
pub struct LnNumIter<'a> {
    ln_num: LnNum,
    ln_iter: Lines<'a>,
}

impl<'a> LnNumIter<'a> {
    /// Start a new iteration.
    pub fn new(text: &'a str) -> Self {
        let ln_num = LnNum::from_pred_count(0);
        let ln_iter = text.lines();
        LnNumIter { ln_num, ln_iter }
    }
}

impl<'a> Iterator for LnNumIter<'a> {
    type Item = (LnNum, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let LnNumIter { ln_num, ln_iter } = self;
        ln_iter.next().map(|line| (*ln_num, line))
    }
}
