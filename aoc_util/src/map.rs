pub struct Map2D<T> {
    map: Vec<T>,
    row_length: usize,
    start_index: usize,
}

impl<T> Map2D<T> {
    pub fn get<U>(&self, p: Point2D<U>) -> &T
    where
        U: Into<usize>,
    {
        let index =
            p.x.into() - self.start_index + (p.y.into() - self.start_index) * self.row_length;
        &self.map[index]
    }
}

pub struct Point2D<T> {
    // column
    pub x: T,
    // row
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
