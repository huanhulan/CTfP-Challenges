pub fn safe_reciprocal(x: f64) -> Option<f64> {
    if x == 0.0 {
        None
    } else {
        Some(1.0 / x)
    }
}

#[cfg(test)]
mod tests {
    use super::safe_reciprocal;

    #[test]
    fn safe_reciprocal_0() {
        assert_eq!(safe_reciprocal(0.0), None);
    }

    #[test]
    fn safe_reciprocal_4() {
        assert_eq!(safe_reciprocal(4.0), Some(0.25));
    }
}
