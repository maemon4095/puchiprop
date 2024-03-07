// [HACK]: use method resolution priority to trait if

#[derive(Debug)]
pub struct TryCloneWrap<'a, T>(pub &'a T);

pub trait TryCloneInner {
    type Inner;

    fn try_clone_inner(&self) -> Option<Self::Inner>;
}

impl<'a, T: Clone> TryCloneWrap<'a, T> {
    pub fn try_clone_inner(&self) -> Option<T> {
        Some(self.0.clone())
    }
}
impl<'a, T> TryCloneInner for TryCloneWrap<'a, T> {
    type Inner = T;

    fn try_clone_inner(&self) -> Option<Self::Inner> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_clone() {
        struct A;
        let a = A;
        let wrap = TryCloneWrap(&a);
        let v = wrap.try_clone_inner();
        assert!(v.is_none());

        #[derive(Clone)]
        struct B;
        let b = B;
        let wrap = TryCloneWrap(&b);
        let v = wrap.try_clone_inner();
        assert!(v.is_some());
    }
}
