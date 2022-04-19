use std::{
  cell::UnsafeCell,
  ops::{Deref, DerefMut},
  sync::atomic::{AtomicBool, Ordering},
};

pub struct Mutex<T: ?Sized> {
  locked: AtomicBool,
  value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
  pub fn new(value: T) -> Self {
    Self {
      locked: AtomicBool::new(false),
      value: UnsafeCell::new(value),
    }
  }
}

impl<T: ?Sized> Mutex<T> {
  pub fn lock(&'_ self) -> MutexGuard<'_, T> {
    while self
      .locked
      .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
      .is_err()
    {
      // spin loop
    }

    MutexGuard { mutex: self }
  }

  fn unlock(&self) {
    self.locked.store(false, Ordering::Release);
  }
}

pub struct MutexGuard<'a, T: ?Sized> {
  mutex: &'a Mutex<T>,
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
  fn drop(&mut self) {
    self.mutex.unlock()
  }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.mutex.value.get() }
  }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.mutex.value.get() }
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use super::*;

  #[test]
  fn smoke() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
      let mutex = Arc::clone(&mutex);

      handles.push(std::thread::spawn(move || {
        for _ in 0..100 {
          let mut value = mutex.lock();
          *value += 1;
        }
      }));
    }

    for handle in handles.into_iter() {
      handle.join().expect("error joining thread");
    }

    assert_eq!(1000, *mutex.lock());
  }
}
