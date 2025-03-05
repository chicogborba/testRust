use std::hash::{Hash, Hasher};
use std::cmp::Ordering;


#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Implementação correta de Hash usando to_bits()
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

// Implementação correta de PartialEq (comparação bit a bit)
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.to_bits() == other.x.to_bits() && 
        self.y.to_bits() == other.y.to_bits()
    }
}

impl Eq for Point {}

// Ordenação segura usando total_cmp (Rust 1.62+)
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.total_cmp(&other.x).then(self.y.total_cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        // Garante que não há NaN ou infinitos
        assert!(x.is_finite() && y.is_finite(), "Pontos devem ser finitos");
        Point { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let mut self_points = [&self.a, &self.b, &self.c];
        let mut other_points = [&other.a, &other.b, &other.c];
        self_points.sort();
        other_points.sort();
        self_points == other_points
    }
}

impl Eq for Triangle {}


impl Hash for Triangle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut points = [&self.a, &self.b, &self.c];
        points.sort();
        points.hash(state);
    }
}