#[cfg(test)]
mod tests {
    use super::super::challenge_5_8_4::{left, right, Either};

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
        assert_eq!(left(m1(i(3))), Some(3));
        assert_eq!(left(m2(i(3))), Some(3));

        assert_eq!(left(m1(i(4))), Some(4));
        assert_eq!(left(m2(i(4))), Some(4));

        assert_eq!(right(m1(j(false))), Some(false));
        assert_eq!(right(m2(j(false))), Some(false));

        assert_eq!(right(m1(j(true))), Some(true));
        assert_eq!(right(m2(j(true))), Some(true));
    }
}
