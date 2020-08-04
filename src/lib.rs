#![doc(html_root_url = "http://docs.rs/const-default/0.1.0")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", feature(const_generics))]
#![cfg_attr(all(feature = "unstable", feature = "alloc"), feature(const_btree_new))]
#![cfg_attr(feature = "unstable", allow(incomplete_features))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "derive")]
pub use const_default_derive::ConstDefault;

pub trait ConstDefault: Sized {
    const DEFAULT: Self;
}

pub trait ConstValue<T> {
    type Output: Sized;

    const VALUE: Self::Output;
}

impl<T> ConstDefault for Option<T> {
    const DEFAULT: Self = None;
}

#[cfg(feature = "alloc")]
impl<'a, T: ConstDefault + Clone + 'a> ConstDefault for alloc::borrow::Cow<'a, T> {
    const DEFAULT: Self = alloc::borrow::Cow::Owned(T::DEFAULT);
}

impl<T: ConstDefault> ConstDefault for core::cell::Cell<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl<T: ConstDefault> ConstDefault for core::cell::UnsafeCell<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl<T: ConstDefault> ConstDefault for core::cell::RefCell<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

// TODO revisit whether this makes sense?
impl<T: ConstDefault> ConstDefault for core::mem::MaybeUninit<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

#[cfg(feature = "alloc")]
impl<T> ConstDefault for alloc::vec::Vec<T> {
    const DEFAULT: Self = Self::new();
}

#[cfg(feature = "alloc")]
impl ConstDefault for alloc::string::String {
    const DEFAULT: Self = Self::new();
}

#[cfg(all(feature = "alloc", feature = "unstable"))]
impl<K: Ord, V> ConstDefault for alloc::collections::BTreeMap<K, V> {
    const DEFAULT: Self = Self::new();
}

#[cfg(all(feature = "alloc", feature = "unstable"))]
impl<T: Ord> ConstDefault for alloc::collections::BTreeSet<T> {
    const DEFAULT: Self = Self::new();
}

#[cfg(feature = "alloc")]
impl<T> ConstDefault for alloc::collections::LinkedList<T> {
    const DEFAULT: Self = Self::new();
}

/*#[cfg(feature = "alloc")]
impl<T: ConstDefault> ConstDefault for alloc::sync::Arc<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

#[cfg(feature = "alloc")]
impl<T: ConstDefault> ConstDefault for alloc::rc::Rc<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

#[cfg(feature = "alloc")]
impl<T: ConstDefault> ConstDefault for alloc::boxed::Box<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}*/

impl<'a, T: 'a> ConstDefault for &'a [T] {
    const DEFAULT: Self = &[];
}

/* Doesn't work :(
impl<'a, T: ConstDefault + 'a> ConstDefault for &'a T {
    const DEFAULT: Self = &T::DEFAULT;
}*/

impl<T> ConstDefault for *const T {
    const DEFAULT: Self = core::ptr::null();
}

impl<T> ConstDefault for *mut T {
    const DEFAULT: Self = core::ptr::null_mut();
}

impl<T: ConstDefault> ConstDefault for core::mem::ManuallyDrop<T> {
    const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl<T: ?Sized> ConstDefault for core::marker::PhantomData<T> {
    const DEFAULT: Self = Self;
}

impl<T> ConstDefault for core::iter::Empty<T> {
    const DEFAULT: Self = core::iter::empty();
}

impl<T: ConstDefault> ConstDefault for core::num::Wrapping<T> {
    const DEFAULT: Self = Self(T::DEFAULT);
}

impl ConstDefault for core::time::Duration {
    const DEFAULT: Self = core::time::Duration::from_secs(0);
}

#[cfg(feature = "std")]
impl ConstDefault for std::sync::Once {
    const DEFAULT: Self = Self::new();
}

macro_rules! impl_num {
    ($($ty:ty=$d:expr$(;$name:ident)?),*) => {
        $(
            impl ConstDefault for $ty {
                const DEFAULT: Self = $d;
            }

            impl ConstDefault for &$ty {
                const DEFAULT: Self = &<$ty as ConstDefault>::DEFAULT;
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
    ()=(), bool=false, f32=0.0, f64=0.0, char='\x00', &str="",
    u8=0;AtomicU8, u16=0;AtomicU16, u32=0;AtomicU32, u64=0;AtomicU64, usize=0;AtomicUsize,
    i8=0;AtomicI8, i16=0;AtomicI16, i32=0;AtomicI32, i64=0;AtomicI64, isize=0;AtomicIsize,
    i128=0, u128=0
}

#[cfg(feature = "std")]
impl ConstDefault for std::sync::atomic::AtomicBool {
    const DEFAULT: Self = Self::new(ConstDefault::DEFAULT);
}

macro_rules! impl_tuple {
    (@rec $t:ident) => { };
    (@rec $_:ident $($t:ident)+) => {
        impl_tuple! { @impl $($t)* }
        impl_tuple! { @rec $($t)* }
    };
    (@impl $($t:ident)*) => {
        impl<$($t: ConstDefault,)*> ConstDefault for ($($t,)*) {
            const DEFAULT: Self = ($($t::DEFAULT,)*);
        }
    };
    ($($t:ident)*) => {
        impl_tuple! { @rec _t $($t)* }
    };
}

impl_tuple! {
    A B C D E F G H I J K L
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
