use std::{
    alloc::{self, alloc, handle_alloc_error, Layout},
    cell::Cell,
    mem::size_of,
    ops::{Deref, Sub},
    ptr::{self, NonNull},
};

pub struct Test {
    // for non 0 size
    pub i: i32,
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("test value dropped");
    }
}

// Does not support Weak counter
#[derive(Debug)]
pub struct RcInner<T> {
    value: T,
    counter: Cell<usize>,
}

#[derive(Debug)]
pub struct Rc<T> {
    pub ptr: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        // Allocate enough memory on the heap to store one T.
        assert_ne!(
            size_of::<T>(),
            0,
            "Zero-sized types are out of the scope of this implementation"
        );
        let ptr = unsafe {
            let layout = Layout::new::<RcInner<T>>();
            let heap_ptr = alloc(layout);
            if heap_ptr.is_null() {
                handle_alloc_error(layout);
            }
            let rc_in = RcInner {
                value,
                counter: Cell::new(1),
            };
            ptr::write::<RcInner<T>>(heap_ptr as *mut RcInner<T>, rc_in);
            println!("{}", 1);
            NonNull::new(heap_ptr as *mut RcInner<T>).expect("Can't be ZST")
        };
        Rc { ptr }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let updated_counter = self.ptr.as_ref().counter.get().sub(1);
            if updated_counter == 0 {
                ptr::drop_in_place(&mut self.ptr.as_mut().value as *mut T);
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::for_value(self.ptr.as_ref()),
                );
            } else {
                self.ptr.as_ref().counter.set(updated_counter);
            }
            println!("{updated_counter}");
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        let ptr = unsafe {
            let counter = &self.ptr.as_ref().counter;
            let updated_counter = counter.get().checked_add(1).expect("Ref counter overflow");
            counter.set(updated_counter);
            println!("{updated_counter}");
            self.ptr
        };
        Rc { ptr }
    }
}
