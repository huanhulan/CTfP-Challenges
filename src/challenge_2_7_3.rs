#[cfg(test)]
mod tests {
    use super::super::challenge_2_7_1::memoize;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn memoize_random() {
        let mut f =
            memoize(|seed: <StdRng as SeedableRng>::Seed| StdRng::from_seed(seed).gen::<i32>());

        assert_eq!(f([1; 32]), f([1; 32]));
        assert_eq!(f([3; 32]), f([3; 32]));
        assert_eq!(f([7; 32]), f([7; 32]));
    }
}
