use std::collections::HashMap;
use std::hash::Hash;

pub fn memoize<T: Hash + Eq + Clone, U: Clone, F: Fn(T) -> U>(f: F) -> impl FnMut(T) -> U {
    let mut map = HashMap::new();

    return move |arg| map.entry(arg.clone()).or_insert_with(|| f(arg)).clone();
}

#[cfg(test)]
mod tests {
    use super::memoize;

    #[test]
    fn memoize_1() {
        let mut f = memoize(|x| x + 7);

        assert_eq!(f(4), 11);
        assert_eq!(f(4), 11);
    }
}
