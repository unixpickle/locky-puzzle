//! Scoped threads with RAII for joining.

use std::marker::PhantomData;
use std::mem::{replace, transmute};
use std::thread::{JoinHandle, spawn};

/// A object that owns a thread.
pub(crate) struct ThreadScope<'a> {
    handle: Option<JoinHandle<()>>,
    phantom: PhantomData<&'a ()>
}

impl<'a> ThreadScope<'a> {
    pub(crate) fn spawn<F: FnOnce() + Send + 'a>(f: F) -> ThreadScope<'a> {
        // Turn our FnOnce into an FnMut to be called from Box.
        let mut f_once = Some(f);
        let caller = move || {
            replace(&mut f_once, None).unwrap()()
        };

        let boxed = Box::new(caller);
        let static_boxed = unsafe {
            transmute::<Box<FnMut() + Send + 'a>, _>(boxed)
        };
        ThreadScope::spawn_static(static_boxed)
    }

    fn spawn_static(mut f: Box<FnMut() + Send + 'static>) -> ThreadScope<'a> {
        let handle: JoinHandle<()>;
        handle = spawn(move || f());
        ThreadScope{handle: Some(handle), phantom: PhantomData}
    }
}

impl<'a> Drop for ThreadScope<'a> {
    fn drop(&mut self) {
        let handle = replace(&mut self.handle, None).unwrap();
        handle.join().unwrap();
    }
}
