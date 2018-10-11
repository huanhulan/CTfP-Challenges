pub fn bimap<A, B, C, D, F: Fn(A) -> C, G: Fn(B) -> D>(f: F, g: G) -> impl Fn((A, B)) -> (C, D) {
    move |(x, y)| (f(x), g(y))
}

pub fn first<A, B, C, F: Fn(A) -> C>(f: F) -> impl Fn((A, B)) -> (C, B) {
    move |(x, y)| (f(x), y)
}

pub fn second<A, B, D, G: Fn(B) -> D>(g: G) -> impl Fn((A, B)) -> (A, D) {
    move |(x, y)| (x, g(y))
}

#[cfg(test)]
mod tests {
    use super::{bimap, first, second};

    fn bool_to_i32(b: bool) -> i32 {
        if b {
            1
        } else {
            0
        }
    }

    fn f32_to_f64(f: f32) -> f64 {
        (f + 1.0) as _
    }

    #[test]
    fn bimap_test() {
        assert_eq!(bimap(bool_to_i32, f32_to_f64)((false, 1.0)), (0, 2.0));
        assert_eq!(bimap(bool_to_i32, f32_to_f64)((true, 4.0)), (1, 5.0));
    }

    #[test]
    fn first_test() {
        assert_eq!(first(bool_to_i32)((false, 1.0)), (0, 1.0));
        assert_eq!(first(bool_to_i32)((true, 4.0)), (1, 4.0));
    }

    #[test]
    fn second_test() {
        assert_eq!(second(f32_to_f64)((false, 1.0)), (false, 2.0));
        assert_eq!(second(f32_to_f64)((true, 4.0)), (true, 5.0));
    }
}
