// [HACK]: use method resolution priority to trait if

use std::ops::Deref;

pub fn try_clone_wrap<'a, T>(r: &'a T) -> TryCloneWrap<'a, T> {
    TryCloneWrap(TryCloneWrapInner(r))
}

pub struct TryCloneWrap<'a, T>(TryCloneWrapInner<'a, T>);
pub struct TryCloneWrapInner<'a, T>(&'a T);

impl<'a, T> TryCloneWrapInner<'a, T> {
    pub fn try_clone_inner(&self) -> Option<T> {
        None
    }
}

impl<'a, T: Clone> TryCloneWrap<'a, T> {
    pub fn try_clone_inner(&self) -> Option<T> {
        Some(self.0 .0.clone())
    }
}

impl<'a, T> Deref for TryCloneWrap<'a, T> {
    type Target = TryCloneWrapInner<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_clone() {
        struct A;
        let a = A;
        let wrap = try_clone_wrap(&a);
        let v = wrap.try_clone_inner();
        assert!(v.is_none());

        #[derive(Clone)]
        struct B;
        let b = B;
        let wrap = try_clone_wrap(&b);
        let v = wrap.try_clone_inner();
        assert!(v.is_some());
    }
}
