pub fn bool_to_bool_identity(x: bool) -> bool {
    return x;
}

pub fn bool_to_bool_not(x: bool) -> bool {
    return !x;
}

pub fn bool_to_bool_true(_: bool) -> bool {
    return true;
}

pub fn bool_to_bool_false(_: bool) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_bool_identity_test() {
        assert_eq!(bool_to_bool_identity(false), false);
        assert_eq!(bool_to_bool_identity(true), true);
    }

    #[test]
    fn bool_to_bool_identity_not() {
        assert_eq!(bool_to_bool_not(false), true);
        assert_eq!(bool_to_bool_not(true), false);
    }

    #[test]
    fn bool_to_bool_identity_true() {
        assert_eq!(bool_to_bool_true(false), true);
        assert_eq!(bool_to_bool_true(true), true);
    }

    #[test]
    fn bool_to_bool_identity_false() {
        assert_eq!(bool_to_bool_false(false), false);
        assert_eq!(bool_to_bool_false(true), false);
    }
}
