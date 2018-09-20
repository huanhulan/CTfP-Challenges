use super::challenge_5_8_4::Either;

pub fn either_t_to_bool_t<T>(e: Either<T, T>) -> (bool, T) {
    match e {
        Either::Left(v) => (false, v),
        Either::Right(v) => (true, v),
    }
}

pub fn bool_t_to_either_t<T>(p: (bool, T)) -> Either<T, T> {
    if p.0 {
        Either::Right(p.1)
    } else {
        Either::Left(p.1)
    }
}

#[cfg(test)]
mod tests {
    use super::{bool_t_to_either_t, either_t_to_bool_t, Either};

    #[test]
    fn bool_t_to_either_t_test() {
        assert_eq!(bool_t_to_either_t((false, 7)), Either::Left(7));
        assert_eq!(bool_t_to_either_t((true, 4)), Either::Right(4));
    }

    #[test]
    fn either_t_to_bool_t_test() {
        assert_eq!(either_t_to_bool_t(Either::Left(7)), (false, 7));
        assert_eq!(either_t_to_bool_t(Either::Right(4)), (true, 4));
    }
}
