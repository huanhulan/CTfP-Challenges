use super::challenge_4_4_2::safe_reciprocal;

pub fn safe_root(x: f64) -> Option<f64> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

pub fn composition<T, U, V, F: Fn(T) -> Option<U>, G: Fn(U) -> Option<V>>(
    g: G,
    f: F,
) -> impl Fn(T) -> Option<V> {
    move |x| match f(x) {
        None => None,
        Some(value) => g(value),
    }
}

pub fn safe_root_reciprocal(x: f64) -> Option<f64> {
    composition(safe_root, safe_reciprocal)(x)
}

#[cfg(test)]
mod tests {
    use super::safe_root_reciprocal;

    #[test]
    fn safe_root_reciprocal_0() {
        assert_eq!(safe_root_reciprocal(0.0), None);
    }

    #[test]
    fn safe_root_reciprocal_negative_one_fourth() {
        assert_eq!(safe_root_reciprocal(-0.25), None);
    }

    #[test]
    fn safe_root_reciprocal_one_fourth() {
        assert_eq!(safe_root_reciprocal(0.25), Some(2.0));
    }
}
