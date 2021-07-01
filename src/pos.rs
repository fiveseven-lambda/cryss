//! エラーに位置情報をもたせるためのモジュール．元のテキストの復元も担う

/// ソースコード中での文字の位置（ `line` 行目， `byte` バイト目）を 0-indexed で表す．
///
/// `derive(Ord)` は，
/// `Range::new()` や `impl Add for Range` において
/// 前後がひっくり返っていないか
/// `debug_assert` する用
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    /// line number
    line: usize,
    /// byte index
    byte: usize,
}

/// ソースコード中でのトークンや式（複数文字／複数行にわたる）の位置を， `start` から `end` までの半開区間として表す．
#[derive(Clone)]
pub struct Range {
    start: Pos,
    end: Pos,
}

impl Pos {
    pub fn new(line: usize, byte: usize) -> Pos {
        Pos { line, byte }
    }
    pub fn into_inner(&self) -> (usize, usize) {
        (self.line, self.byte)
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
    /// エラーが起こっている行を出力．
    pub fn print(&self, log: &Vec<String>) {
        let (line, byte) = self.into_inner();
        print!("{} !-> {}", &log[line][..byte], &log[line][byte..]);
    }
}
impl Range {
    /// エラーが起こっている行を出力．
    pub fn print(&self, log: &Vec<String>) {
        let start = &self.start;
        let end = &self.end;
        if start.line == end.line {
            // 一行の場合
            print!(
                "{} !-> {} <-! {}",
                &log[start.line][..start.byte],
                &log[start.line][start.byte..end.byte],
                &log[end.line][end.byte..]
            );
        } else {
            // 複数行にわたる場合
            print!("{} !-> {}", &log[start.line][..start.byte], &log[start.line][start.byte..]);
            for row in &log[start.line + 1..end.line] {
                print!("{}", row);
            }
            print!("{} <-! {}", &log[end.line][..end.byte], &log[end.line][end.byte..]);
        }
    }
}

use std::ops::Add;
/// A, B を式やトークンとし，位置がそれぞれ `a: Range`，`b: Range` として得られているとする．ソースコード内で B が A より後にあるとき， `a + b` で AB を合わせた範囲が得られる．
impl Add for Range {
    type Output = Range;
    fn add(self, other: Range) -> Range {
        debug_assert!(self.end <= other.start);
        Range::new(self.start, other.end)
    }
}
/// `a + b` において， `b` が `None` であれば `a` 自体を返す．
impl Add<Option<Range>> for Range {
    type Output = Range;
    fn add(self, right: Option<Range>) -> Range {
        match right {
            Some(right) => self + right,
            None => self,
        }
    }
}
/// `a + b` において， `a` が `None` であれば `b` 自体を返す．
impl Add<Range> for Option<Range> {
    type Output = Range;
    fn add(self, right: Range) -> Range {
        match self {
            Some(left) => left + right,
            None => right,
        }
    }
}
