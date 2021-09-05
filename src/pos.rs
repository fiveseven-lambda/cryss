//! エラーに位置情報をもたせるためのモジュール．
//!
//! 使用時： `use crate::pos`

/// ソースコード中の文字の位置を表す．
///
/// `line` 行目， `byte` バイト目の組．
/// 内部では 0-indexed．
///
/// `derive(Clone)`：
/// 式の `Range` を，構成する式やトークンの `Range` から得る際に使う
///
/// `derive(Ord)`：
/// `Range::new()` や `Range: std::ops::Add` において
/// 前後がひっくり返っていないか assert する用
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    line: usize,
    byte: usize,
}

/// ソースコード中のトークンや式の位置を表す．
///
/// 始まり `start: Pos` と終わり `end: Pos` の組．
/// `start` を含み `end` を含まない半開区間．
pub struct Range {
    start: Pos,
    end: Pos,
}

impl Pos {
    pub fn new(line: usize, byte: usize) -> Pos {
        Pos { line, byte }
    }
    pub fn byte(&self) -> usize {
        self.byte
    }
}

impl Range {
    pub fn new(start: Pos, end: Pos) -> Range {
        debug_assert!(start <= end);
        Range { start, end }
    }
}

use std::fmt::{self, Debug, Display, Formatter};
/// 1-indexed に直して出力する．
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.byte + 1)
    }
}
/// 0-indexed のまま出力する．
impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.byte)
    }
}
/// 1-indexed，閉区間に直して出力する．
impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.start.line + 1,
            self.start.byte + 1,
            self.end.line + 1,
            self.end.byte
        )
    }
}
/// 0-indexed，半開区間のまま出力する．
impl Debug for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?})", self.start, self.end)
    }
}

impl Pos {
    pub fn eprint(&self, log: &[String]) {
        let &Pos { line, byte } = self;
        eprint!("{} !-> {}", &log[line][..byte], &log[line][byte..])
    }
}
impl Range {
    pub fn eprint(&self, log: &[String]) {
        let Range { start, end } = self;
        if start.line == end.line {
            eprint!(
                "{} !-> {} <-! {}",
                &log[start.line][..start.byte],
                &log[start.line][start.byte..end.byte],
                &log[end.line][end.byte..]
            );
        } else {
            eprint!(
                "{} !-> {}",
                &log[start.line][..start.byte],
                &log[start.line][start.byte..]
            );
            for row in &log[start.line + 1..end.line] {
                eprint!("{}", row);
            }
            eprint!(
                "{} <-! {}",
                &log[end.line][..end.byte],
                &log[end.line][end.byte..]
            )
        }
    }
}

use std::ops::Add;
/// A, B を式やトークンとし，位置がそれぞれ `a: Range`，`b: Range` として得られているとする．ソースコード内で B が A より後にあるとき， `a + b` で AB を合わせた範囲が得られる．
impl Add<Range> for Range {
    type Output = Range;
    fn add(self, other: Range) -> Range {
        debug_assert!(self.end <= other.start);
        Range::new(self.start, other.end)
    }
}
impl Add<&Range> for Range {
    type Output = Range;
    fn add(self, other: &Range) -> Range {
        debug_assert!(self.end <= other.start);
        Range::new(self.start, other.end.clone())
    }
}
impl Add<Range> for &Range {
    type Output = Range;
    fn add(self, other: Range) -> Range {
        debug_assert!(self.end <= other.start);
        Range::new(self.start.clone(), other.end)
    }
}
impl Add<&Range> for &Range {
    type Output = Range;
    fn add(self, other: &Range) -> Range {
        debug_assert!(self.end <= other.start);
        Range::new(self.start.clone(), other.end.clone())
    }
}
