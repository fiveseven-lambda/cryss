#[derive(Clone)]
pub struct Start {
    line: usize,
    column: usize,
}

#[derive(Clone)]
pub struct End {
    line: usize,
    column: Option<usize>,
}

#[derive(Clone)]
pub struct Range {
    start: Start,
    end: End,
}

impl Start {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
impl End {
    pub fn new(line: usize, column: Option<usize>) -> Self {
        Self { line, column }
    }
}
impl Range {
    pub fn new(start: Start, end: End) -> Self {
        Self { start, end }
    }

    pub fn new_single_line(line: usize, start: usize, end: Option<usize>) -> Range {
        Range {
            start: Start {
                line,
                column: start,
            },
            end: End { line, column: end },
        }
    }
}

use std::fmt::{self, Debug, Display, Formatter};
impl Display for Start {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
impl Debug for Start {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl Display for End {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:", self.line + 1)?;
        match self.column {
            Some(column) => write!(f, "{}", column),
            None => write!(f, "$"),
        }
    }
}
impl Debug for End {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:", self.line)?;
        match self.column {
            Some(column) => write!(f, "{}", column),
            None => write!(f, "$"),
        }
    }
}
impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
impl Debug for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}", self.start, self.end)
    }
}

impl Start {
    pub fn eprint(&self, log: &[String]) {
        let line = &log[self.line];
        eprint!("{} !-> {}", &line[..self.column], &line[self.column..]);
    }
}

impl Range {
    pub fn eprint(&self, log: &[String]) {
        if self.start.line == self.end.line {
            let line = &log[self.start.line];
            let start = self.start.column;
            match self.end.column {
                Some(end) => {
                    eprint!(
                        "{} !-> {} <-! {}",
                        &line[..start],
                        &line[start..end],
                        &line[end..]
                    )
                }
                None => {
                    eprintln!("{} !-> {} <-!", &line[..start], &line[start..])
                }
            }
        } else {
            let sline = &log[self.start.line];
            let start = self.start.column;
            eprint!("{} !-> {}", &sline[..start], &sline[start..]);
            eprintln!("...");
            let eline = &log[self.end.line];
            match self.end.column {
                Some(end) => {
                    eprint!("{} <-! {}", &eline[..end], &eline[end..])
                }
                None => {
                    eprintln!("{} <-!", &eline)
                }
            }
        }
    }
}

impl std::ops::Add<Range> for Range {
    type Output = Range;
    fn add(self, rhs: Range) -> Range {
        Range::new(self.start, rhs.end)
    }
}
impl std::ops::Add<&Range> for Range {
    type Output = Range;
    fn add(self, rhs: &Range) -> Range {
        Range::new(self.start, rhs.end.clone())
    }
}
impl std::ops::Add<Range> for &Range {
    type Output = Range;
    fn add(self, rhs: Range) -> Range {
        Range::new(self.start.clone(), rhs.end)
    }
}
impl std::ops::Add<&Range> for &Range {
    type Output = Range;
    fn add(self, rhs: &Range) -> Range {
        Range::new(self.start.clone(), rhs.end.clone())
    }
}
// impl std::ops::AddAssign<Range> for Range {
//     fn add_assign(&mut self, rhs: Range) {
//         self.end = rhs.end;
//     }
// }
// impl std::ops::AddAssign<&Range> for Range {
//     fn add_assign(&mut self, rhs: &Range) {
//         self.end = rhs.end.clone();
//     }
// }
//
