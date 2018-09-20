use super::challenge_5_8_4::Either;

pub fn maybe_to_either<T>(m: Option<T>) -> Either<(), T> {
    match m {
        None => Either::Left(()),
        Some(v) => Either::Right(v),
    }
}

pub fn either_to_maybe<T>(e: Either<(), T>) -> Option<T> {
    match e {
        Either::Left(()) => None,
        Either::Right(v) => Some(v),
    }
}

#[cfg(test)]
mod tests {
    use super::{either_to_maybe, maybe_to_either, Either};

    #[test]
    fn maybe_to_either_test() {
        assert_eq!(maybe_to_either::<i32>(None), Either::Left(()));
        assert_eq!(maybe_to_either(Some(6)), Either::Right(6));
    }

    #[test]
    fn either_to_maybe_test() {
        assert_eq!(either_to_maybe(Either::Left::<(), i32>(())), None);
        assert_eq!(either_to_maybe(Either::Right(14)), Some(14));
    }
}
