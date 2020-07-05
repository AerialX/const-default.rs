#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", feature(const_generics))]
#![cfg_attr(feature = "unstable", allow(incomplete_features))]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::mem::MaybeUninit;
use core::cell::{Cell, UnsafeCell, RefCell};

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

#[cfg(feature = "alloc")]
impl<T: ConstDefault> ConstDefault for alloc::sync::Arc<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

#[cfg(feature = "alloc")]
impl<T: ConstDefault> ConstDefault for alloc::rc::Rc<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

#[cfg(feature = "std")]
impl<T> ConstDefault for Vec<T> {
    const DEFAULT: Self = Vec::new();
}

#[cfg(feature = "std")]
impl ConstDefault for String {
    const DEFAULT: Self = String::new();
}

impl<T: ConstDefault> ConstDefault for core::mem::ManuallyDrop<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl<T: ?Sized> ConstDefault for core::marker::PhantomData<T> {
    const DEFAULT: Self = Self;
}

impl<T: ConstDefault> ConstDefault for core::num::Wrapping<T> {
    const DEFAULT: Self = Self(T::DEFAULT);
}

macro_rules! impl_num {
    ($($ty:ty$(;$name:ident)?),*) => {
        $(
            impl ConstDefault for $ty {
                const DEFAULT: Self = 0;
            }

            $(
                #[cfg(feature = "std")]
                impl ConstDefault for std::sync::atomic::$name {
                    const DEFAULT: Self = Self::new(ConstDefault::DEFAULT);
                }
            )?
        )*
    };
}

impl_num! {
    u8;AtomicU8, u16;AtomicU16, u32;AtomicU32, u64;AtomicU64, usize;AtomicUsize,
    i8;AtomicI8, i16;AtomicI16, i32;AtomicI32, i64;AtomicI64, isize;AtomicIsize,
    i128, u128
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

impl ConstDefault for char {
    const DEFAULT: Self = '\x00';
}

#[cfg(feature = "std")]
impl ConstDefault for std::sync::atomic::AtomicBool {
    const DEFAULT: Self = Self::new(ConstDefault::DEFAULT);
}

impl ConstDefault for () {
    const DEFAULT: Self = ();
}

#[cfg(not(feature = "unstable"))]
macro_rules! impl_array {
    ($($len:tt),*) => {
        $(impl<T: ConstDefault> ConstDefault for [T; $len] {
            const DEFAULT: Self = [T::DEFAULT; $len];
        })*
    };
}

#[cfg(not(feature = "unstable"))]
impl_array! {
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32
}

#[cfg(feature = "unstable")]
impl<T: ConstDefault, const N: usize> ConstDefault for [T; N] {
    const DEFAULT: Self = [T::DEFAULT; N];
}
