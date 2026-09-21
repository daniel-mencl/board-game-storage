use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinates {
    row: usize,
    col: usize,
}

pub fn same_row(first: &Coordinates, second: &Coordinates) -> bool {
    first.row == second.row
}

pub fn same_col(first: &Coordinates, second: &Coordinates) -> bool {
    first.col == second.col
}

pub struct Board2D<T> {
    size: usize,
    tiles: Vec<T>,
}

impl<T: Clone> Board2D<T> {
    pub fn new(size: usize, tile: T) -> Board2D<T> {
        let tiles = vec![tile; size * size];
        Board2D { size, tiles }
    }
}

impl<T> Board2D<T> {
    fn coords_to_index(&self, coords: Coordinates) -> Option<usize> {
        if coords.row >= self.size || coords.col >= self.size {
            None
        } else {
            Some(coords.row * self.size + coords.col)
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

    pub fn get(&self, coords: Coordinates) -> Option<&T> {
        self.coords_to_index(coords).map(|index| &self.tiles[index])
    }
}
