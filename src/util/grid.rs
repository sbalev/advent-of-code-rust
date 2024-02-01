use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<T>,
}

impl<T: From<u8>> Grid<T> {
    pub fn parse(input: &str, border: u8) -> Self {
        let input: Vec<_> = input.lines().collect();
        let m = input.len() + 2;
        let n = input[0].len() + 2;
        let mut cells = Vec::with_capacity(m * n);

        for _ in 0..n {
            cells.push(T::from(border))
        }
        for line in input {
            cells.push(T::from(border));
            for b in line.bytes() {
                cells.push(T::from(b));
            }
            cells.push(T::from(border));
        }
        for _ in 0..n {
            cells.push(T::from(border))
        }

        Self { m, n, cells }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index * self.n..(index + 1) * self.n]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index * self.n..(index + 1) * self.n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = Grid::<u8>::parse("123\n456", b'.');
        assert_eq!(4, grid.m);
        assert_eq!(5, grid.n);
        for i in 0..grid.m {
            assert_eq!(b'.', grid[i][0]);
            assert_eq!(b'.', grid[i][grid.n - 1]);
        }
        for j in 0..grid.n {
            assert_eq!(b'.', grid[0][j]);
            assert_eq!(b'.', grid[grid.m - 1][j])
        }
        for i in 1..grid.m - 1 {
            for j in 1..grid.n - 1 {
                assert_eq!(b'0' + ((j + (i - 1) * (grid.n - 2)) as u8), grid[i][j])
            }
        }
        grid[1][2] = b'8';
        assert_eq!(b'8', grid[1][2]);
    }
}
