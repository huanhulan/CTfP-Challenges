use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn circ(&self) -> f64;
}

pub struct Circle {
    r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }

    fn circ(&self) -> f64 {
        2.0 * PI * self.r
    }
}

pub struct Rect {
    d: f64,
    h: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.d * self.h
    }

    fn circ(&self) -> f64 {
        2.0 * (self.d + self.h)
    }
}

pub struct Suqare {
    d: f64,
}

impl Shape for Suqare {
    fn area(&self) -> f64 {
        self.d * self.d
    }

    fn circ(&self) -> f64 {
        4.0 * self.d
    }
}

#[cfg(test)]
mod tests {
    use super::{Circle, Rect, Shape, Suqare, PI};

    #[test]
    fn circle_area() {
        assert_eq!(Circle { r: 2.0 }.area(), PI * 4.0)
    }

    #[test]
    fn rect_area() {
        assert_eq!(Rect { d: 2.0, h: 3.0 }.area(), 6.0)
    }

    #[test]
    fn square_area() {
        assert_eq!(Suqare { d: 4.0 }.area(), 16.0)
    }

    #[test]
    fn circle_circ() {
        assert_eq!(Circle { r: 2.0 }.circ(), PI * 4.0)
    }

    #[test]
    fn rect_circ() {
        assert_eq!(Rect { d: 2.0, h: 3.0 }.circ(), 10.0)
    }

    #[test]
    fn square_circ() {
        assert_eq!(Suqare { d: 3.0 }.circ(), 12.0)
    }

}
