use std::cell::RefCell;

pub mod cpp_arena;
pub mod rust_bench;

pub struct Arena<T> {
    entries: RefCell<Vec<T>>,
}

impl<T> Arena<T> {
    /// Create new arena with the given capacity
    pub fn new(capacity: usize) -> Arena<T> {
        Arena {
            entries: RefCell::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn has_capacity(&self) -> bool {
        let chunks = self.entries.borrow();

        chunks.len() < chunks.capacity()
    }

    /// Return an immutable reference to object inside the entries vector
    /// references will share a lifetime/have lifetimes that are as long as each other
    #[inline]
    pub fn alloc(&self, value: T) -> & T {
        let mut chunks = self.entries.borrow_mut();
        let len = chunks.len();

        if len < chunks.capacity() {
            chunks.push(value);
            unsafe { & *chunks.as_mut_ptr().add(len) }
        } else {
            panic!()
        }
    }
}
