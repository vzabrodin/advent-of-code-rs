pub struct Grid<T>
where
    T: Copy,
{
    buffer: Vec<T>,
    rows: usize,
    columns: usize,
}

#[allow(dead_code)]
impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(buffer: Vec<T>, rows: usize, columns: usize) -> Grid<T> {
        assert!(buffer.len() == rows * columns);
        Grid {
            buffer,
            rows,
            columns,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn row(&self, row: usize) -> Option<&[T]> {
        let row_offset = self.columns * row;
        let index = self.columns * row + self.columns;
        if self.buffer.len() > index {
            Some(&self.buffer[row_offset..(row_offset + self.columns)])
        } else {
            None
        }
    }

    pub fn iter_row(&self, row: usize) -> GridRowIterator<T> {
        GridRowIterator {
            grid: &self,
            row,
            column: 0,
        }
    }

    pub fn iter_column(&self, column: usize) -> GridColumnIterator<T> {
        GridColumnIterator {
            grid: &self,
            row: 0,
            column,
        }
    }

    pub fn slice_row(&self, row: usize) -> &[T] {
        let row_index = self.columns * row;
        &self.buffer[row_index..row_index + self.columns()]
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        let row_offset = self.columns * row;
        self.buffer.get(row_offset + column)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        let row_offset = self.columns * row;
        self.buffer.get_mut(row_offset + column)
    }
}

pub struct GridRowIterator<'a, T>
where
    T: Copy,
{
    grid: &'a Grid<T>,
    row: usize,
    column: usize,
}

impl<'a, T> Iterator for GridRowIterator<'a, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column == self.grid.columns() {
            return None;
        }

        let item = self.grid.get(self.row, self.column).and_then(|x| Some(*x));
        self.column += 1;
        item
    }
}

impl<'a, T> DoubleEndedIterator for GridRowIterator<'a, T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.column == self.grid.columns() {
            return None;
        }

        let item = self
            .grid
            .get(self.row, self.grid.columns() - self.column - 1)
            .and_then(|x| Some(*x));
        self.column += 1;
        item
    }
}

pub struct GridColumnIterator<'a, T>
where
    T: Copy,
{
    grid: &'a Grid<T>,
    row: usize,
    column: usize,
}

impl<'a, T> Iterator for GridColumnIterator<'a, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.grid.rows() {
            return None;
        }

        let item = self.grid.get(self.row, self.column).and_then(|x| Some(*x));
        self.row += 1;
        item
    }
}

impl<'a, T> DoubleEndedIterator for GridColumnIterator<'a, T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row == self.grid.rows() {
            return None;
        }

        let item = self
            .grid
            .get(self.grid.rows() - self.row - 1, self.column)
            .and_then(|x| Some(*x));
        self.row += 1;
        item
    }
}
