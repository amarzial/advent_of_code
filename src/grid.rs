use num::Signed;

pub fn manhattan<T: Signed>(p1: (T, T), p2: (T, T)) -> T {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub struct Grid<T>
where
    T: Copy,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn from_iter<I>(iter: I, width: usize, height: usize) -> Grid<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut g = Grid {
            data: Vec::new(),
            width,
            height,
        };
        g.data.extend(iter);
        if g.data.len() != width * height {
            panic!("grid of wrong size");
        }
        g
    }
}

impl<T> IntoIterator for Grid<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = GridIntoIterator<T>;
    fn into_iter(self) -> GridIntoIterator<T> {
        GridIntoIterator {
            grid: self,
            index: 0,
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter_row(self, row: usize) -> GridRowIntoIterator<T> {
        if row >= self.height {
            panic!("trying to iterate over a non existing row");
        }
        GridRowIntoIterator {
            grid: self,
            index: 0,
            row,
        }
    }

    pub fn iter_column(self, column: usize) -> GridColumnIntoIterator<T> {
        if column >= self.width {
            panic!("trying to iterate over a non existing column");
        }
        GridColumnIntoIterator {
            grid: self,
            index: 0,
            column,
        }
    }
}

pub struct GridIntoIterator<T>
where
    T: Copy,
{
    grid: Grid<T>,
    index: usize,
}

impl<T> Iterator for GridIntoIterator<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.grid.data.len() {
            let v = self.grid.data[self.index];
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

pub struct GridRowIntoIterator<T>
where
    T: Copy,
{
    grid: Grid<T>,
    index: usize,
    row: usize,
}

impl<T> Iterator for GridRowIntoIterator<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.grid.width {
            let v = self.grid.data[self.index + self.row * self.grid.width];
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}

pub struct GridColumnIntoIterator<T>
where
    T: Copy,
{
    grid: Grid<T>,
    index: usize,
    column: usize,
}

impl<T> Iterator for GridColumnIntoIterator<T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.grid.height {
            let v = self.grid.data[self.index * self.grid.height + self.column];
            self.index += 1;
            Some(v)
        } else {
            None
        }
    }
}
