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

#[derive(Debug)]
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

pub fn array<G, const N: usize>(generators: [G; N]) -> Array<G, N>
where
    G: TestCaseGenerator,
{
    Array { generators }
}

#[derive(Debug)]
pub struct Array<G: TestCaseGenerator, const N: usize> {
    generators: [G; N],
}

impl<G: TestCaseGenerator, const N: usize> TestCaseGenerator for Array<G, N> {
    type TestCase = [G::TestCase; N];

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        std::array::from_fn(|i| self.generators[i].generate(rng))
    }
}

pub fn vec<G, R>(generator: G, len: R) -> Vec<G, R>
where
    G: TestCaseGenerator,
    R: Clone + SampleRange<usize>,
{
    Vec { generator, len }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub struct Constant<T: Debug + Clone>(T);
impl<T: Debug + Clone> TestCaseGenerator for Constant<T> {
    type TestCase = T;

    fn generate(&self, _rng: &mut dyn RngCore) -> Self::TestCase {
        self.0.clone()
    }
}

pub fn zip<G0, G1>(generator0: G0, generator1: G1) -> Zip<G0, G1>
where
    G0: TestCaseGenerator,
    G1: TestCaseGenerator,
{
    Zip(generator0, generator1)
}

#[derive(Debug)]
pub struct Zip<G0, G1>(G0, G1)
where
    G0: TestCaseGenerator,
    G1: TestCaseGenerator;

impl<G0, G1> TestCaseGenerator for Zip<G0, G1>
where
    G0: TestCaseGenerator,
    G1: TestCaseGenerator,
{
    type TestCase = (G0::TestCase, G1::TestCase);

    fn generate(&self, rng: &mut dyn RngCore) -> Self::TestCase {
        (self.0.generate(rng), self.1.generate(rng))
    }
}

#[macro_export]
macro_rules! tuple {
    ($($e:expr),*) => {{
        #[allow(unused_variables)]
        move |rng: &mut dyn ::rand::RngCore| {
            ($($e.generate(rng)),*)
        }
    }};
}

pub use tuple;

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
        cases! {
            [
                [], [0], [0, 0], [0, 0, 0], [0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ] => |p| {
                array(p.map(|e: usize| constant(e)))
            } => |case, p| {
                assert_eq!(case, p)
            }
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

    #[test]
    fn test_zip() {
        cases! {
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            ] => |p| {
                zip(constant(p), constant(p))
            } => |case, p| {
                assert_eq!(case, &(*p, *p))
            }
        }
    }

    macro_rules! test_tuple_n {
        ($($e: expr),*) => {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            test_tuple_n!(@acc rng; $($e,)*);
        };
        (@acc $rng: expr; $first: expr, $($e: expr,)* ) => {
            test_tuple_n!(@test $rng; $first, $($e,)*);
            test_tuple_n!(@acc $rng; $($e,)*);
        };
        (@acc $rng: expr;) => {
            test_tuple_n!(@test $rng;);
        };
        (@test $rng: expr; $($e: expr,)*) => {
            let g = tuple!($(constant($e)),*);
            for _ in 0..10 {
                let case = g.generate(&mut $rng);
                assert_eq!(case, ($($e),*));
            }
        }
    }

    #[test]
    fn test_tuple() {
        test_tuple_n!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    }
}
