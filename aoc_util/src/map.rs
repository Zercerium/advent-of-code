#[derive(Clone)]
pub struct Map2D<T> {
    map: Vec<T>,
    row_length: usize,
    start_index: usize,
    x_torus: bool,
    y_torus: bool,
}

// pls be aware this is just for AoC and not for production use,
// so there are no checks for out of bonds requests, cause i don't want to return Results or Options
// happy panicking
impl<T> Map2D<T>
where
    T: Clone,
{
    pub fn new_default(x_size: usize, y_size: usize, default: T, start_index: usize) -> Self {
        let size = x_size * y_size;
        let mut map = Vec::with_capacity(size);
        (0..size).for_each(|_| map.push(default.clone()));
        Map2D {
            map,
            row_length: x_size,
            start_index,
            x_torus: false,
            y_torus: false,
        }
    }

    pub fn new(row_length: usize) -> Self {
        Map2D {
            map: vec![],
            row_length,
            start_index: 0,
            x_torus: false,
            y_torus: false,
        }
    }

    pub fn append_row(&mut self, items: &mut Vec<T>) {
        assert_eq!(self.row_length, items.len());
        self.map.append(items);
    }

    pub fn set_x_torus(&mut self) {
        self.x_torus = true;
    }

    pub fn set_y_torus(&mut self) {
        self.y_torus = true;
    }

    fn index<U>(&self, p: Point2D<U>) -> usize
    where
        U: Into<usize> + Copy,
    {
        let mut x: usize = p.x.into() - self.start_index;
        if self.x_torus {
            x %= self.row_length
        };
        assert!(x < self.row_length);
        x + (p.y.into() - self.start_index) * self.row_length
    }

    pub fn get<U>(&self, p: Point2D<U>) -> &T
    where
        U: Into<usize> + Copy,
    {
        let index = self.index(p);
        &self.map[index]
    }

    pub fn set<U>(&mut self, p: Point2D<U>, element: T)
    where
        U: Into<usize> + Copy,
    {
        let index = self.index(p);
        self.map[index] = element;
    }

    pub fn columns(&self) -> usize {
        self.map.len() / self.row_length
    }
}

#[derive(Clone, Copy)]
pub struct Point2D<T>
where
    T: Copy,
{
    // column
    pub x: T,
    // row
    pub y: T,
}

impl<T> Point2D<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn add_x(&mut self, x: T) {
        self.x = self.x + x;
    }

    pub fn add_y(&mut self, y: T) {
        self.y = self.y + y;
    }

    pub fn add(&mut self, a: (T, T)) {
        self.x = self.x + a.0;
        self.y = self.y + a.1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        let mut map = Map2D::new_default(4, 3, 0, 1);
        map.set(Point2D::<u8>::new(1, 1), 1);
        map.set(Point2D::<u8>::new(4, 3), 9);
        map.set(Point2D::<u8>::new(2, 2), 7);
        map.set(Point2D::<u8>::new(4, 1), 6);

        #[rustfmt::skip]
        let vec = vec![
        1,0,0,6,
        0,7,0,0,
        0,0,0,9];

        assert_eq!(map.map, vec);

        map.set_x_torus();
        assert_eq!(*map.get(Point2D::<u8>::new(8, 1)), 6);

        assert_eq!(map.columns(), 3);
    }
}
