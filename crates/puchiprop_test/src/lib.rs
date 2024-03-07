pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use puchiprop::prelude::*;
    use rand::Rng;

    #[prop_test(|rng| (rng.gen_range(0..100), rng.gen_range(0..100)), options = { seed: 8274166976581544106, skip: 6 })]
    fn it_works(a: usize, b: usize) {
        let result = add(a, b);
        assert!(result < 150)
    }

    #[prop_test(|_| A)]
    fn cannot_clone(_cannot_clone: A) {}

    #[derive(Debug)]
    struct A;
}
