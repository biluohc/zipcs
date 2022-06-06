pub struct TakedDefer<F: FnOnce()>(Option<F>);
pub trait Take {
    type T;
    fn take(&mut self) -> Option<Self::T>;
}

impl<T: FnOnce()> Take for TakedDefer<T> {
    type T = T;
    fn take(&mut self) -> Option<Self::T> {
        self.0.take()
    }
}

impl<F: FnOnce()> Drop for TakedDefer<F> {
    fn drop(&mut self) {
        self.take().map(|f| f());
    }
}

/// defer execute the closure until the drop
pub fn defer<F: FnOnce()>(f: F) -> impl Drop + Take<T = F> {
    TakedDefer(Some(f))
}

#[test]
fn test() {
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
