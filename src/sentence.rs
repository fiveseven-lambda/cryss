use crate::expr;

#[derive(Debug)]
pub enum Sentence {
    Expr(Option<expr::PExpr>),
}

pub enum PreSentence {
    Expr(Option<expr::PPreExpr>),
}

impl From<PreSentence> for Sentence {
    fn from(pre_sentence: PreSentence) -> Sentence {
        match pre_sentence {
            PreSentence::Expr(expr) => Sentence::Expr(expr.map(|expr| {
                let expr = (expr.0, expr.1.into());
                expr.into()
            })),
        }
    }
}

use crate::pos;
pub type PPreSentence = (pos::Range, PreSentence);
pub type PSentence = (pos::Range, Sentence);
