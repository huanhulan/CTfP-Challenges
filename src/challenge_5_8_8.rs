#[cfg(test)]
mod tests {
    use super::super::challenge_5_8_4::Either;

    fn i(n: i32) -> (Either<i32, bool>, Either<i32, bool>) {
        (Either::Left(n), Either::Left(n))
    }

    fn j(b: bool) -> (Either<i32, bool>, Either<i32, bool>) {
        (Either::Right(b), Either::Right(b))
    }

    fn m1(e: (Either<i32, bool>, Either<i32, bool>)) -> Either<i32, bool> {
        e.0
    }

    fn m2(e: (Either<i32, bool>, Either<i32, bool>)) -> Either<i32, bool> {
        e.1
    }

    #[test]
    fn either_either_is_not_better() {
        assert_eq!(m1(i(3)), Either::Left(3));
        assert_eq!(m2(i(3)), Either::Left(3));

        assert_eq!(m1(i(4)), Either::Left(4));
        assert_eq!(m2(i(4)), Either::Left(4));

        assert_eq!(m1(j(false)), Either::Right(false));
        assert_eq!(m2(j(false)), Either::Right(false));

        assert_eq!(m1(j(true)), Either::Right(true));
        assert_eq!(m2(j(true)), Either::Right(true));
    }
}
