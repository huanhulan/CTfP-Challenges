#[cfg(test)]
mod tests {
    use super::super::challenge_2_7_1::memoize;
    use rand::random;

    #[test]
    fn memoize_random() {
        let mut f = memoize(|_: ()| random::<i32>());
        let r0 = f(());
        let r1 = f(());
        let r2 = f(());
        let r3 = f(());
        let r4 = f(());
        let r5 = f(());

        assert_eq!(r0, r1);
        assert_eq!(r0, r2);
        assert_eq!(r0, r3);
        assert_eq!(r0, r4);
        assert_eq!(r0, r5);
    }
}
