pub struct TakedDefer<O, F: FnOnce() -> O>(Option<F>);
pub trait Take {
    type T;
    fn take(&mut self) -> Option<Self::T>;
}

impl<O, T: FnOnce() -> O> Take for TakedDefer<O, T> {
    type T = T;
    fn take(&mut self) -> Option<Self::T> {
        self.0.take()
    }
}

impl<O, F: FnOnce() -> O> Drop for TakedDefer<O, F> {
    fn drop(&mut self) {
        self.take().map(|f| f());
    }
}

/// defer execute the closure until the drop
pub fn defer<O, F: FnOnce() -> O>(f: F) -> impl Drop + Take<T = F> {
    TakedDefer(Some(f))
}

#[test]
fn test_defer() {
    use std::cell::RefCell;

    let i = RefCell::new(0);

    {
        let _d = defer(|| *i.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 0);
    }
    assert_eq!(*i.borrow(), 1);

    {
        let mut _d = defer(|| *i.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 1);
        _d.take();
    }
    assert_eq!(*i.borrow(), 1);
}

#[test]
fn test_defer_output() {
    use std::sync::atomic::{AtomicU8, Ordering};

    let x = AtomicU8::new(0);
    let l = || x.load(Ordering::Acquire);
    let i = || x.fetch_add(1, Ordering::Release);

    assert_eq!(l(), 0);
    {
        let _d = defer(|| i());
        assert_eq!(l(), 0);
    }
    assert_eq!(l(), 1);

    {
        let mut _d = defer(i);
        (_d.take().unwrap())();
    }
    assert_eq!(l(), 2);
}
