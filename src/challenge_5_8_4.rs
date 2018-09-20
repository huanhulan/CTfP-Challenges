#[derive(Debug, PartialEq)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

#[cfg(test)]
mod tests {
    use super::Either;

    #[test]
    fn either_left() {
        assert_eq!(Either::Left::<bool, i32>(true), Either::Left(true));
    }

    #[test]
    fn either_right() {
        assert_eq!(Either::Right::<bool, i32>(7), Either::Right(7));
    }
}
