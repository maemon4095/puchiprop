pub fn genfn<T, F: for<'a> Fn(&'a mut dyn rand::RngCore) -> T>(f: F) -> F {
    f
}
