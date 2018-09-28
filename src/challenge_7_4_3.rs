use super::challenge_1_4_2::composition;

fn fmap<A, B, R, F: Fn(A) -> B, G: Fn(R) -> A>(f: F, g: G) -> impl Fn(R) -> B {
    composition(f, g)
}

#[cfg(test)]
mod tests {
    use super::super::challenge_1_4_1::identity;
    use super::{composition, fmap};

    fn bool_to_int(b: bool) -> i32 {
        if b {
            1
        } else {
            0
        }
    }

    fn int_to_float(x: i32) -> f64 {
        (x * x) as _
    }

    fn float_to_vec(x: f64) -> Vec<f64> {
        vec![x]
    }

    #[test]
    fn map_identity() {
        assert_eq!(fmap(identity, bool_to_int)(false), bool_to_int(false));
        assert_eq!(fmap(identity, bool_to_int)(true), bool_to_int(true));
    }

    #[test]
    fn map_compose() {
        assert_eq!(
            fmap(composition(float_to_vec, int_to_float), bool_to_int)(false),
            composition(|x| fmap(float_to_vec, x), |x| fmap(int_to_float, x))(bool_to_int)(false)
        );

        assert_eq!(
            fmap(composition(float_to_vec, int_to_float), bool_to_int)(true),
            composition(|x| fmap(float_to_vec, x), |x| fmap(int_to_float, x))(bool_to_int)(true)
        );
    }
}
