use puchiprop_core::*;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng, RngCore,
};
use std::{fmt::Debug, marker::PhantomData};

pub fn range<T, R>(range: R) -> Range<T, R>
where
    T: Debug + SampleUniform,
    R: SampleRange<T> + Clone,
{
    Range {
        range,
        marker: PhantomData,
    }
}

pub struct Range<T: Debug + SampleUniform, R: SampleRange<T> + Clone> {
    range: R,
    marker: PhantomData<T>,
}

impl<T: Debug + SampleUniform, R: SampleRange<T> + Clone> TestCaseGenerator for Range<T, R> {
    type TestCase = T;

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        rng.gen_range(self.range.clone())
    }
}

pub fn array<G, const N: usize>(generator: G) -> Array<G, N>
where
    G: TestCaseGenerator,
{
    Array { generator }
}

pub struct Array<G: TestCaseGenerator, const N: usize> {
    generator: G,
}

impl<G: TestCaseGenerator, const N: usize> TestCaseGenerator for Array<G, N> {
    type TestCase = [G::TestCase; N];

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        std::array::from_fn(|_| self.generator.generate(rng))
    }
}

pub fn vec<G, R>(generator: G, len: R) -> Vec<G, R>
where
    G: TestCaseGenerator,
    R: Clone + SampleRange<usize>,
{
    Vec { generator, len }
}

pub struct Vec<G, R>
where
    G: TestCaseGenerator,
    R: Clone + SampleRange<usize>,
{
    generator: G,
    len: R,
}

impl<G, R> TestCaseGenerator for Vec<G, R>
where
    G: TestCaseGenerator,
    R: Clone + SampleRange<usize>,
{
    type TestCase = std::vec::Vec<G::TestCase>;

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        let len = rng.gen_range(self.len.clone());
        let mut vec = std::vec::Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.generator.generate(rng));
        }
        vec
    }
}

pub fn choice<G: TestCaseGenerator, A: AsRef<[G]>>(cases: A) -> Choice<G, A> {
    if cases.as_ref().len() == 0 {
        panic!("no cases was given");
    }

    Choice {
        cases,
        marker: PhantomData,
    }
}

pub struct Choice<G, T>
where
    G: TestCaseGenerator,
    T: AsRef<[G]>,
{
    cases: T,
    marker: PhantomData<G>,
}

impl<G, T> TestCaseGenerator for Choice<G, T>
where
    G: TestCaseGenerator,
    T: AsRef<[G]>,
{
    type TestCase = G::TestCase;

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        let items = self.cases.as_ref();
        let idx = rng.gen_range(0..items.len());
        items[idx].generate(rng)
    }
}

pub fn constant<T: Debug + Clone>(item: T) -> Constant<T> {
    Constant(item)
}

pub struct Constant<T: Debug + Clone>(T);
impl<T: Debug + Clone> TestCaseGenerator for Constant<T> {
    type TestCase = T;

    fn generate(&self, _rng: &mut dyn RngCore) -> Self::TestCase {
        self.0.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::SmallRng, SeedableRng};

    macro_rules! cases {
        ([$($c: expr),*] => |$param0: ident| $g: block => |$case: ident, $param1: ident| $t: block) => {
            let mut rng = SmallRng::from_entropy();
            $({
                let param = $c;
                let gen = {
                    let $param0 = param.clone();
                    $g
                };
                for _ in 0..100 {
                    let case = gen.generate(&mut rng);
                    let $case = &case;
                    let $param1 = &param;
                    $t
                }
            })*
        };
    }

    #[test]
    fn test_range() {
        cases! {
            [
                0..10,
                0.0..1.0
            ] => |p| {
                range(p)
            } => |r, p| {
                assert!(p.contains(r))
            }
        }
    }

    #[test]
    fn test_constant() {
        cases! {
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            ] => |p| {
                constant(p)
            } => |case, p| {
                assert_eq!(case, p);
            }
        }
    }

    #[test]
    fn test_array() {
        test_array_n::<0>();
        test_array_n::<1>();
        test_array_n::<2>();
        test_array_n::<4>();
        test_array_n::<8>();
        test_array_n::<16>();
        test_array_n::<32>();
        test_array_n::<64>();
    }

    fn test_array_n<const N: usize>() {
        let g = array::<_, N>(constant(0));
        let mut rng = SmallRng::from_entropy();
        for _ in 0..10 {
            let case = g.generate(&mut rng);
            assert_eq!(case, [0; N])
        }
    }

    #[test]
    fn test_vec() {
        cases! {
            [
                0..10,
                5..10,
                10..=10
            ] => |p| {
                vec(constant(0), p)
            } => |case, p| {
                assert!(p.contains(&case.len()));
            }
        }
    }

    #[test]
    fn test_choice() {
        cases! {
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            ] => |p| {
                let choices: std::vec::Vec<_> = (0..p).into_iter().map(constant).collect();
                choice(choices)
            } => |case, p| {
                assert!(case < p);
            }
        }
    }
}
