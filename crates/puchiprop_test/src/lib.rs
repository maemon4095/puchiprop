pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use puchiprop::{helper::genfn, prelude::*};
    use rand::Rng;

    #[prop_test(|rng| (rng.gen_range(0..100), rng.gen_range(0..100)), options = { seed: 8274166976581544106, skip: 6 })]
    fn it_works(a: usize, b: usize) {
        let result = add(a, b);
        assert!(result < 150)
    }

    fn array<G>(
        g: G,
        len: impl rand::distributions::uniform::SampleRange<usize> + Clone,
    ) -> impl TestCaseGenerator<TestCase = Vec<G::TestCase>>
    where
        G: TestCaseGenerator,
    {
        genfn(move |rng| {
            let len = rng.gen_range(len.clone());
            let mut buf = Vec::with_capacity(len);
            for _ in 0..len {
                buf.push(g.generate(rng));
            }
            buf
        })
    }

    #[prop_test(array(genfn(|r| r.gen()), 0..10))]
    fn takes_array(_items: Vec<usize>) {}

    #[prop_test(|_| A)]
    fn cannot_clone(_cannot_clone: A) {}

    #[derive(Debug)]
    struct A;
}
