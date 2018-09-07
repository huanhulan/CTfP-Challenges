pub fn composition<T, U, V, F: Fn(T) -> U, G: Fn(U) -> V>(g: G, f: F) -> impl Fn(T) -> V {
    return move |x| g(f(x));
}

#[cfg(test)]
mod tests {
    use super::composition;

    #[test]
    fn composition_1() {
        assert_eq!(composition(|x| x * 2, |x| x + 3)(3), 12);
    }

    #[test]
    fn composition_2() {
        assert_eq!(composition(|x| x + 3, |x| x * 2)(3), 9);
    }
}
