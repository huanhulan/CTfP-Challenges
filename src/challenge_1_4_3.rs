#[cfg(test)]
mod tests {
    use super::super::challenge_1_4_1::identity;
    use super::super::challenge_1_4_2::composition;

    #[test]
    fn composition_1() {
        assert_eq!(composition(identity, |x| x + 3)(2), 5);
    }

    #[test]
    fn composition_2() {
        assert_eq!(composition(|x| x + 3, identity)(2), 5);
    }
}
