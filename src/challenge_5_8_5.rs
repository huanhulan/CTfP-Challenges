#[cfg(test)]
mod tests {
    use super::super::challenge_5_8_4::Either;

    fn i(n: i32) -> i32 {
        n
    }

    fn j(b: bool) -> i32 {
        if b {
            0
        } else {
            1
        }
    }

    fn m(e: Either<i32, bool>) -> i32 {
        match e {
            Either::Left(v) => i(v),
            Either::Right(v) => j(v),
        }
    }

    #[test]
    fn either_is_better() {
        assert_eq!(m(Either::Left(3)), i(3));
        assert_eq!(m(Either::Left(4)), i(4));
        assert_eq!(m(Either::Right(true)), j(true));
        assert_eq!(m(Either::Right(false)), j(false));
    }
}
