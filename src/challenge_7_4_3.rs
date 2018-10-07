pub struct MappedReader<F, RA> {
    func: F,
    ra: RA,
}

impl<A, B, R, F: Fn(A) -> B, RA: Fn(R) -> A> FnOnce<(R,)> for MappedReader<F, RA> {
    type Output = B;

    extern "rust-call" fn call_once(self, r: (R,)) -> Self::Output {
        (self.func)((self.ra)(r.0))
    }
}

impl<A, B, R, F: Fn(A) -> B, RA: Fn(R) -> A> FnMut<(R,)> for MappedReader<F, RA> {
    extern "rust-call" fn call_mut(&mut self, r: (R,)) -> Self::Output {
        (self.func)((self.ra)(r.0))
    }
}

impl<A, B, R, F: Fn(A) -> B, RA: Fn(R) -> A> Fn<(R,)> for MappedReader<F, RA> {
    extern "rust-call" fn call(&self, r: (R,)) -> Self::Output {
        (self.func)((self.ra)(r.0))
    }
}

pub struct ReaderFunc<F> {
    func: F,
}

impl<F, RA> FnOnce<(RA,)> for ReaderFunc<F> {
    type Output = MappedReader<F, RA>;

    extern "rust-call" fn call_once(self, ra: (RA,)) -> Self::Output {
        MappedReader {
            func: self.func,
            ra: ra.0,
        }
    }
}

impl<F: Clone, RA> FnMut<(RA,)> for ReaderFunc<F> {
    extern "rust-call" fn call_mut(&mut self, ra: (RA,)) -> Self::Output {
        MappedReader {
            func: self.func.clone(),
            ra: ra.0,
        }
    }
}

impl<F: Clone, RA> Fn<(RA,)> for ReaderFunc<F> {
    extern "rust-call" fn call(&self, ra: (RA,)) -> Self::Output {
        MappedReader {
            func: self.func.clone(),
            ra: ra.0,
        }
    }
}

pub fn fmap<F>(func: F) -> ReaderFunc<F> {
    ReaderFunc { func }
}

#[cfg(test)]
mod tests {
    use super::super::challenge_1_4_1::identity;
    use super::super::challenge_1_4_2::composition;
    use super::fmap;

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
        assert_eq!(fmap(identity)(bool_to_int)(false), bool_to_int(false));
        assert_eq!(fmap(identity)(bool_to_int)(true), bool_to_int(true));
    }

    #[test]
    fn map_compose() {
        assert_eq!(
            fmap(composition(float_to_vec, int_to_float))(bool_to_int)(false),
            composition(fmap(float_to_vec), fmap(int_to_float))(bool_to_int)(false)
        );

        assert_eq!(
            fmap(composition(float_to_vec, int_to_float))(bool_to_int)(true),
            composition(fmap(float_to_vec), fmap(int_to_float))(bool_to_int)(true)
        );
    }
}
