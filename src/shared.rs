use core::{cell::UnsafeCell, ops::{Deref, DerefMut}};
use core::ffi::c_void;

pub trait WdfObject {
    unsafe fn from_ptr(inner: WDFOBJECT) -> Self;
    fn as_ptr(&self) -> WDFOBJECT;
}

pub struct SharedRef<T: WdfObject> {
    wdf_obj: UnsafeCell<T>,
    spin_lock: WDFSPINLOCK, // TODO: will have to support other kinds of locks as well
}

impl<T: WdfObject> SharedRef<T> {
    pub fn new(wdf_obj: T) -> Self {
        let spin_lock = Self::get_spin_lock(&wdf_obj);

        // Increment WDF reference with WdfObjectReferenceActual

        Self { wdf_obj: UnsafeCell::new(wdf_obj), spin_lock }
    }

    pub fn lock_spin(&self) -> SharedRefGuard<T> {
        SharedRefGuard::lock_and_create(self)
    }

    pub fn as_ptr(&self) -> WDFOBJECT {
        unsafe { (*self.wdf_obj.get()).as_ptr() }
    }

    fn get_spin_lock(_wdf_obj: &T) -> WDFSPINLOCK {
        // TODO: Get spin lock from wdf_obj's context
        // if not available, create it and add it to the  context.

        unimplemented!()
    }
}

impl<T: WdfObject> Clone for SharedRef<T> {
    fn clone(&self) -> Self {
        // Increment WDF reference with WdfObjectReferenceActual
        Self { wdf_obj: unsafe { T::from_ptr(self.as_ptr()) }.into(), spin_lock: self.spin_lock }
    }
}

impl<T: WdfObject> Drop for SharedRef<T> {
    fn drop(&mut self) {
        // Decrement WDF reference with WdfObjectDereferenceActual
    }
}

pub struct SharedRefGuard<'a, T: WdfObject + 'a> {
    shared: &'a SharedRef<T>,
}

impl<'a, T: WdfObject> SharedRefGuard<'a, T> {
    fn lock_and_create(shared: &'a SharedRef<T>) -> Self {
        // Acquire spinlock here
        Self { shared }
    }
}

impl<'a, T: WdfObject> Drop for SharedRefGuard<'a, T> {
    fn drop(&mut self) {
        // Release spinlock here
    }
}

impl<'a, T: WdfObject> Deref for SharedRefGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.shared.wdf_obj.get() }
    }
}

impl<'a, T: WdfObject> DerefMut for SharedRefGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut*self.shared.wdf_obj.get() }
    }
}

// TODO: DANGER - these have not yet been properly verified
unsafe impl<T: Send + Sync + WdfObject> Sync for SharedRef<T> {}
unsafe impl<T: Send + Sync + WdfObject> Send for SharedRef<T> {}
unsafe impl<T: Sync + WdfObject> Sync for SharedRefGuard<'_, T> {}
unsafe impl<T: Send + WdfObject> Send for SharedRefGuard<'_, T> {}


type WDFOBJECT = *mut c_void;
type WDFSPINLOCK = *mut c_void;

#[allow(non_camel_case_types)]
struct WDF_OBJECT_ATTRIBUTES;

impl Default for WDF_OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unimplemented!()
    }
}