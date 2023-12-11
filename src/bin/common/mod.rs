use std::{fmt, str::FromStr};

use num::abs;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Coords {
    pub line: i64,
    pub col: i64,
}

impl std::ops::Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            line: self.line + rhs.line,
            col: self.col + rhs.col,
        }
    }
}

impl Coords {
    pub fn dist_square(&self, other: &Coords) -> i64 {
        abs(other.line - self.line) + abs(other.col - self.col)
    }
}

#[derive(PartialEq, Eq)]
pub struct Map<T>(pub Vec<Vec<T>>);

impl<T: From<char>> FromStr for Map<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect()))
    }
}

impl<T: From<char>> From<&str> for Map<T> {
    fn from(s: &str) -> Self {
        s.parse().expect("Error parsing universe")
    }
}

impl<T: fmt::Debug> fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for line in self.0.iter() {
            for c in line {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Map<T> {
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if coords.line < 0 || coords.col < 0 {
            return None;
        }
        self.0
            .get(usize::try_from(coords.line).ok()?)?
            .get(usize::try_from(coords.col).ok()?)
    }
}
