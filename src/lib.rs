#![cfg_attr(not(feature = "std"), no_std)]

use core::mem::MaybeUninit;
use core::cell::{Cell, UnsafeCell, RefCell};
#[cfg(feature = "std")]
use std::{
    rc::Rc,
    sync::Arc,
};

pub trait ConstDefault {
    const DEFAULT: Self;
}

pub trait ConstValue<T = Self> {
    const VALUE: T;
}

impl<T: ConstDefault> ConstValue<T> for T {
    const VALUE: T = T::DEFAULT;
}

impl<T> ConstDefault for Option<T> {
    const DEFAULT: Self = None;
}

impl<T: ConstDefault> ConstDefault for Cell<T> {
    const DEFAULT: Self = Cell::new(T::DEFAULT);
}

impl<T: ConstDefault> ConstDefault for UnsafeCell<T> {
    const DEFAULT: Self = UnsafeCell::new(T::DEFAULT);
}

impl<T: ConstDefault> ConstDefault for RefCell<T> {
    const DEFAULT: Self = RefCell::new(T::DEFAULT);
}

// TODO revisit whether this makes sense?
impl<T: ConstDefault> ConstDefault for MaybeUninit<T> {
    const DEFAULT: Self = MaybeUninit::new(T::DEFAULT);
}

#[cfg(feature = "std")]
impl<T: ConstDefault> ConstDefault for Arc<T> {
    const DEFAULT: Self = Arc::new(T::DEFAULT);
}

#[cfg(feature = "std")]
impl<T: ConstDefault> ConstDefault for Rc<T> {
    const DEFAULT: Self = Rc::new(T::DEFAULT);
}

// TODO lots, i/u128, atomics, Vec, String (when did that become stable?)

macro_rules! impl_num {
    ($($ty:ty),*) => {
        $(impl ConstDefault for $ty {
            const DEFAULT: Self = 0;
        })*
    };
}

impl_num! {
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize
}

impl ConstDefault for f32 {
    const DEFAULT: Self = 0.0;
}

impl ConstDefault for f64 {
    const DEFAULT: Self = 0.0;
}

impl ConstDefault for bool {
    const DEFAULT: Self = false;
}
