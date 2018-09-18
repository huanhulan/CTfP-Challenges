pub enum Either<T, U> {
    Left(T),
    Right(U),
}

pub fn left<T, U>(value: Either<T, U>) -> Option<T> {
    match value {
        Either::Left(v) => Some(v),
        _ => None,
    }
}

pub fn right<T, U>(value: Either<T, U>) -> Option<U> {
    match value {
        Either::Right(v) => Some(v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{left, right, Either};

    #[test]
    fn either_left() {
        assert_eq!(left(Either::Left::<bool, i32>(true)), Some(true));
        assert_eq!(right(Either::Left::<bool, i32>(true)), None);
    }

    #[test]
    fn either_right() {
        assert_eq!(left(Either::Right::<bool, i32>(7)), None);
        assert_eq!(right(Either::Right::<bool, i32>(7)), Some(7));
    }
}
