use core::{
    mem::transmute,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct AtomicMutex<T: ?Sized> {
    locked: AtomicBool,
    inner: T,
}

unsafe impl<T: Sized> Send for AtomicMutex<T> {}
unsafe impl<T: Sized> Sync for AtomicMutex<T> {}

pub struct AtomicMutexGuard<'a, T: ?Sized> {
    mutex: &'a AtomicMutex<T>,
    inner: &'a mut T,
}

impl<T> AtomicMutex<T> {
    pub const fn new(t: T) -> AtomicMutex<T> {
        AtomicMutex {
            locked: AtomicBool::new(false),
            inner: t,
        }
    }
}

impl<T: ?Sized> AtomicMutex<T> {
    pub fn lock(&self) -> AtomicMutexGuard<'_, T> {
        let inner_ptr = &self.inner as *const T;

        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {}

        // SAFETY: we know that this will be the only reference
        let inner_mut_ptr: *mut T = unsafe { transmute(inner_ptr) };
        AtomicMutexGuard {
            mutex: self,
            inner: unsafe { inner_mut_ptr.as_mut().unwrap() },
        }
    }
}

impl<'a, T: ?Sized> Drop for AtomicMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: ?Sized> Deref for AtomicMutexGuard<'a, T> {
    fn deref(&self) -> &Self::Target {
        self.inner
    }

    type Target = T;
}

impl<'a, T: ?Sized> DerefMut for AtomicMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}
