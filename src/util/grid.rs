pub struct Grid<T: From<u8>> {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<T>,
}

impl<T: From<u8>> Grid<T> {
    pub fn parse(input: &str) -> Self {
        let input: Vec<_> = input.lines().collect();
        let m = input.len();
        let n = input[0].len();
        let mut cells = Vec::with_capacity(m * n);

        for line in input {
            for b in line.bytes() {
                cells.push(T::from(b));
            }
        }

        Self { m, n, cells }
    }

    pub fn is_inside(&self, i: usize, j: usize) -> bool {
        0 < i && i <= self.m && 0 < j && j <= self.n
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.cells[(i - 1) * self.n + j - 1]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.cells[(i - 1) * self.n + j - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::<u8>::parse("123\n456");
        assert_eq!(2, grid.m);
        assert_eq!(3, grid.n);
        assert_eq!(&b'1', grid.get(1, 1));

        assert!(grid.is_inside(1, 1));
        assert!(!grid.is_inside(0, 1));
        assert!(!grid.is_inside(1, 0));
        assert!(!grid.is_inside(0, 0));
        assert!(grid.is_inside(1, 3));
        assert!(!grid.is_inside(1, 4));
    }
}
