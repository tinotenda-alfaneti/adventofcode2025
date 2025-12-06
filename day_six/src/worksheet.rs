use anyhow::{anyhow, Result};

pub struct Worksheet {
    pub lines: Vec<String>,
    pub width: usize,
}

impl Worksheet {
    /// Normalize the given lines in-place (pads them) and return a Worksheet
    /// containing a clone of the normalized lines. This preserves the caller's
    /// buffer mutation behavior while keeping Worksheet ownership local.
    pub fn from_mut(lines: &mut Vec<String>) -> Self {
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        for l in lines.iter_mut() {
            if l.len() < width {
                l.push_str(&" ".repeat(width - l.len()));
            }
        }
        Worksheet { lines: lines.clone(), width }
    }
    
    pub fn column_groups(&self) -> Vec<ColumnGroup> {
        let mut groups = Vec::new();
        let mut current: Vec<Vec<char>> = Vec::new();

        for col in 0..self.width {
            let col_chars: Vec<char> = self.lines.iter().map(|l| l.chars().nth(col).unwrap()).collect();

            let is_empty = col_chars.iter().all(|c| *c == ' ');

            if is_empty {
                if !current.is_empty() {
                    groups.push(ColumnGroup { cols: current });
                    current = Vec::new();
                }
            } else {
                current.push(col_chars);
            }
        }

        if !current.is_empty() {
            groups.push(ColumnGroup { cols: current });
        }

        groups
    }
}

pub struct ColumnGroup {
    pub cols: Vec<Vec<char>>,
}

impl ColumnGroup {
    pub fn to_rows(&self) -> Vec<String> {
        if self.cols.is_empty() {
            return vec![];
        }
        let height = self.cols[0].len();
        let width = self.cols.len();
        let mut rows = vec![String::new(); height];
        for r in 0..height {
            for c in 0..width {
                rows[r].push(self.cols[c][r]);
            }
        }
        rows
    }

    pub fn parse_problem<S: ParseStrategy>(&self, strategy: &S) -> Result<Problem> {
        let rows = self.to_rows();
        let bottom = rows.last().unwrap().trim();
        if bottom != "+" && bottom != "*" {
            return Err(anyhow!("Invalid operator: {}", bottom));
        }
        let op = bottom.chars().next().unwrap();
        let numbers = strategy.parse_numbers(self)?;
        Ok(Problem { numbers, op })
    }
}

pub trait ParseStrategy {
    fn parse_numbers(&self, cg: &ColumnGroup) -> Result<Vec<i64>>;
}

pub struct LeftToRight;
pub struct RightToLeft;

impl ParseStrategy for LeftToRight {
    fn parse_numbers(&self, cg: &ColumnGroup) -> Result<Vec<i64>> {
        // Left-to-right interpretation: each ROW (except the bottom operator row)
        // is a number read left-to-right across the group's columns.
        let rows = cg.to_rows();
        if rows.is_empty() {
            return Ok(vec![]);
        }
        let mut out = Vec::new();
        for row in rows[..rows.len()-1].iter() {
            let t = row.trim();
            if !t.is_empty() {
                out.push(t.parse::<i64>()?);
            }
        }
        Ok(out)
    }
}

impl ParseStrategy for RightToLeft {
    fn parse_numbers(&self, cg: &ColumnGroup) -> Result<Vec<i64>> {
        if cg.cols.is_empty() {
            return Ok(vec![]);
        }
        let height = cg.cols[0].len();
        let mut out = Vec::new();
        for c in (0..cg.cols.len()).rev() {
            let mut s = String::new();
            for r in 0..(height - 1) {
                s.push(cg.cols[c][r]);
            }
            let t = s.trim();
            if !t.is_empty() {
                out.push(t.parse::<i64>()?);
            }
        }
        Ok(out)
    }
}

pub struct Problem {
    pub numbers: Vec<i64>,
    pub op: char,
}

impl Problem {
    pub fn result(&self) -> i64 {
        match self.op {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => unreachable!("only + or * expected"),
        }
    }
}
