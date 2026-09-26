use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl Coordinates {
    pub fn diagonal_index(&self) -> isize {
        (self.row as isize) - (self.col as isize)
    }

    pub fn antidiagonal_index(&self) -> usize {
        self.row + self.col
    }

    pub fn same_row(first: &Coordinates, second: &Coordinates) -> bool {
        first.row == second.row
    }

    pub fn same_col(first: &Coordinates, second: &Coordinates) -> bool {
        first.col == second.col
    }

    pub fn diagonally_adjacent(first: &Coordinates, second: &Coordinates) -> bool {
        first.row.abs_diff(second.row) <= 1 && first.col.abs_diff(second.col) <= 1
    }

    pub fn orthogonal_distance(first: &Coordinates, second: &Coordinates) -> usize {
        first.row.abs_diff(second.row) + first.col.abs_diff(second.col)
    }

    pub fn orthogonally_adjacent(first: &Coordinates, second: &Coordinates) -> bool {
        Self::orthogonal_distance(first, second) <= 1
    }

    pub fn same_diagonal(first: &Coordinates, second: &Coordinates) -> bool {
        first.diagonal_index() == second.diagonal_index()
    }

    pub fn same_antidiagonal(first: &Coordinates, second: &Coordinates) -> bool {
        first.antidiagonal_index() == second.antidiagonal_index()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board2D<T> {
    pub size: usize,
    tiles: Vec<T>,
}

impl<T: Clone + Default> Board2D<T> {
    pub fn new(size: usize) -> Board2D<T> {
        let tiles = vec![T::default(); size * size];
        Board2D { size, tiles }
    }
}

impl<T> Board2D<T> {
    const ORTHOGONAL_OFFSETS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    const DIAGONAL_OFFSETS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    fn coords_to_index(&self, coord: Coordinates) -> Option<usize> {
        if coord.row >= self.size || coord.col >= self.size {
            None
        } else {
            Some(coord.row * self.size + coord.col)
        }
    }

    fn index_to_coords(&self, index: usize) -> Option<Coordinates> {
        if index >= self.size * self.size {
            None
        } else {
            Some(Coordinates {
                row: index / self.size,
                col: index % self.size,
            })
        }
    }

    fn all_indices(&self) -> Vec<usize> {
        (0..self.tiles.len()).collect()
    }

    fn all_coordinates(&self) -> Vec<Coordinates> {
        let mut result = Vec::new();
        result.reserve(self.size * self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                result.push(Coordinates { row, col });
            }
        }

        result
    }

    pub fn tile_coordinates(&self, predicate: impl Fn(&T) -> bool) -> Vec<Coordinates> {
        let mut result = Vec::new();

        for index in 0..self.tiles.len() {
            if predicate(&self.tiles[index]) {
                result.push(self.index_to_coords(index).unwrap());
            }
        }

        result
    }

    pub fn components<C, N>(
        &self,
        coords: C,
        neighbor_function: impl Fn(&Coordinates) -> N,
    ) -> Vec<HashSet<Coordinates>>
    where
        C: IntoIterator<Item = Coordinates>,
        N: IntoIterator<Item = Coordinates>,
    {
        let mut components = Vec::new();
        let mut unvisited: HashSet<_> = coords.into_iter().collect();

        while let Some(&start) = unvisited.iter().next() {
            unvisited.remove(&start);

            let mut queue = VecDeque::new();
            queue.push_back(start);

            let mut component = HashSet::new();

            while let Some(current) = queue.pop_front() {
                let neighbors = neighbor_function(&current);

                for neighbor in neighbors {
                    if unvisited.remove(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }

                component.insert(current);
            }

            components.push(component);
        }

        components
    }

    pub fn get(&self, coord: Coordinates) -> Option<&T> {
        self.coords_to_index(coord).map(|index| &self.tiles[index])
    }

    fn offset(&self, coord: &Coordinates, offsets: (isize, isize)) -> Option<Coordinates> {
        let (row_offset, col_offset) = offsets;
        let new_row = coord.row as isize + row_offset;
        if new_row < 0 || new_row > self.size as isize {
            return None;
        }

        let new_col = coord.col as isize + col_offset;
        if new_col < 0 || new_col > self.size as isize {
            return None;
        }

        Some(Coordinates {
            row: new_row as usize,
            col: new_col as usize,
        })
    }

    fn offsets_from(
        &self,
        coord: &Coordinates,
        all_offsets: impl IntoIterator<Item = (isize, isize)>,
    ) -> Vec<Coordinates> {
        all_offsets
            .into_iter()
            .filter_map(|offset| self.offset(coord, offset))
            .collect()
    }

    pub fn orthogonal_neighbors(&self, coord: &Coordinates) -> Vec<Coordinates> {
        self.offsets_from(coord, Self::ORTHOGONAL_OFFSETS)
    }

    pub fn diagonal_neighbors(&self, coord: &Coordinates) -> Vec<Coordinates> {
        self.offsets_from(coord, Self::DIAGONAL_OFFSETS)
    }

    pub fn count_tiles(
        &self,
        coords: impl IntoIterator<Item = Coordinates>,
        predicate: impl Fn(&T) -> bool,
    ) -> usize {
        coords
            .into_iter()
            .filter_map(|coord| self.get(coord))
            .filter(|tile| predicate(tile))
            .count()
    }

    pub fn count_all_tiles(&self, predicate: impl Fn(&T) -> bool) -> usize {
        let coords = self.all_coordinates();
        coords
            .into_iter()
            .filter_map(|coord| self.get(coord))
            .filter(|tile| predicate(tile))
            .count()
    }

    pub fn row(&self, row: usize) -> Vec<Coordinates> {
        (0..self.size)
            .into_iter()
            .map(|col| Coordinates { row, col })
            .collect()
    }

    pub fn col(&self, col: usize) -> Vec<Coordinates> {
        (0..self.size)
            .into_iter()
            .map(|row| Coordinates { row, col })
            .collect()
    }

    pub fn diagonal(&self, diagonal: isize) -> Vec<Coordinates> {
        self.all_coordinates()
            .into_iter()
            .filter(|coord| coord.diagonal_index() == diagonal)
            .collect()
    }

    pub fn antidiagonal(&self, antidiagonal: usize) -> Vec<Coordinates> {
        self.all_coordinates()
            .into_iter()
            .filter(|coord| coord.antidiagonal_index() == antidiagonal)
            .collect()
    }

    pub fn line(&self, coord: Coordinates, offsets: (isize, isize)) -> Vec<Coordinates> {
        let mut result = Vec::new();

        while let Some(new_coord) = self.offset(&coord, offsets) {
            result.push(new_coord);
        }

        result
    }

    pub fn orthogonal_lines(&self, coord: Coordinates) -> Vec<Vec<Coordinates>> {
        Self::ORTHOGONAL_OFFSETS
            .into_iter()
            .map(|offset| self.line(coord, offset))
            .collect()
    }
}
