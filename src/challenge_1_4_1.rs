pub fn identity<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::identity;

    #[test]
    fn identity_of_3_is_3() {
        assert_eq!(identity(3), 3);
    }

    #[test]
    fn identity_of_true_is_true() {
        assert_eq!(identity(true), true);
    }
}
