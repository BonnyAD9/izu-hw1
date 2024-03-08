use anyhow::Result;
use std::{io::BufRead, iter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CellType {
    Normal,
    Start,
    Finish,
}

#[derive(Copy, Clone, Debug)]
pub struct MapCell {
    pub typ: CellType,
    pub price: u32,
}

pub struct TransMap {
    width: usize,
    height: usize,
    cells: Vec<Option<MapCell>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl TransMap {
    pub fn iter(&self) -> impl Iterator<Item = (Point, Option<MapCell>)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| (Point::new(i % self.width, i / self.width), *c))
    }

    pub fn at(&self, Point { x, y }: Point) -> Option<MapCell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells[y * self.width + x]
        }
    }

    pub fn from_file<R>(r: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut width = 0;
        let mut table = vec![];

        for line in r.lines() {
            let line = line?;
            let v: Vec<_> = line
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut s = s;
                    let typ = if s.starts_with('S') {
                        s = &s[1..];
                        CellType::Start
                    } else if s.starts_with('F') {
                        s = &s[1..];
                        CellType::Finish
                    } else if s.starts_with(|c: char| c.is_ascii_digit()) {
                        CellType::Normal
                    } else {
                        return None;
                    };

                    let price = s.parse().ok()?;
                    Some(MapCell { typ, price })
                })
                .collect();
            if !v.is_empty() {
                width = width.max(v.len());
                table.push(v);
            }
        }

        let mut cells = vec![];
        let height = table.len();

        for mut row in table {
            let len = row.len();
            cells.append(&mut row);
            cells.extend(iter::repeat(None).take(width - len));
        }

        Ok(TransMap {
            width,
            height,
            cells,
        })
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn surround(&self) -> impl Iterator<Item = Point> {
        let &Self { x, y } = self;
        [
            Self::new(x - 1, y - 1),
            Self::new(x, y - 1),
            Self::new(x + 1, y - 1),
            Self::new(x - 1, y),
            Self::new(x + 1, y),
            Self::new(x - 1, y + 1),
            Self::new(x, y + 1),
            Self::new(x + 1, y + 1),
        ]
        .into_iter()
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}
