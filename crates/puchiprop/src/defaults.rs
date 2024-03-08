use puchiprop_core::*;
use rand::SeedableRng;

#[derive(Debug, Default)]
pub struct DefaultTestPlanner;

#[derive(Debug, Default)]
pub struct DefaultTestPlannerOptions {
    sample_count: Option<usize>,
    seed: Option<u64>,
    skip: Option<usize>,
}

impl DefaultTestPlannerOptions {
    pub fn sample_count(&mut self, limit: usize) {
        self.sample_count = Some(limit);
    }

    pub fn seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }

    pub fn skip(&mut self, skip: usize) {
        self.skip = Some(skip);
    }
}

impl TestPlanner for DefaultTestPlanner {
    type PlanOptions = DefaultTestPlannerOptions;

    fn default_options(&self) -> Self::PlanOptions {
        Self::PlanOptions::default()
    }

    fn plan<G: TestCaseGenerator>(
        &self,
        options: &Self::PlanOptions,
        generator: &G,
    ) -> impl TestPlan<G::TestCase> {
        let seed = options.seed.unwrap_or_else(|| rand::random());
        let skip = options.skip.unwrap_or(0);
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let iterator = std::iter::repeat_with(move || generator.generate(&mut rng))
            .skip(skip)
            .take(options.sample_count.unwrap_or(100));
        DefaultTestPlan {
            seed,
            executed_test_count: skip,
            iterator,
        }
    }
}

struct DefaultTestPlan<I: Iterator> {
    seed: u64,
    executed_test_count: usize,
    iterator: I,
}

impl<I: Iterator<Item = T>, T> TestPlan<T> for DefaultTestPlan<I> {
    fn report_state(&self, f: &mut dyn TestPlanStateReporter) {
        f.report("seed", &self.seed);
        f.report("index", &(self.executed_test_count - 1));
    }
}

impl<I: Iterator> Iterator for DefaultTestPlan<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(e) = self.iterator.next() else {
            return None;
        };
        self.executed_test_count += 1;
        Some(e)
    }
}
