// this file is copied and modified from rustc

pub trait Idx: Sized + Copy + 'static {
    fn new(idx: usize) -> Self;

    fn index(self) -> usize;

    fn inc_one(&mut self) -> Self {
        let old = *self;
        *self = self.plus(1);
        old
    }

    fn increment_by(&mut self, amount: usize) {
        *self = self.plus(amount);
    }

    #[must_use]
    fn plus(self, amount: usize) -> Self {
        Self::new(self.index() + amount)
    }
}

#[macro_export]
macro_rules! newtype_index {
    // ---- public rules ----

    // Use default constants
    ($(#[$attrs:meta])* $v:vis struct $name:ident { .. }) => (
        $crate::newtype_index!(
            // Leave out derives marker so we can use its absence to ensure it comes first
            @attrs        [$(#[$attrs])*]
            @type         [$name]
            // shave off 256 indices at the end to allow space for packing these indices into enums
            @max          [0xFFFF_FF00]
            @vis          [$v]
            @debug_format ["{}"]);
    );

    // Define any constants
    ($(#[$attrs:meta])* $v:vis struct $name:ident { $($tokens:tt)+ }) => (
        $crate::newtype_index!(
            // Leave out derives marker so we can use its absence to ensure it comes first
            @attrs        [$(#[$attrs])*]
            @type         [$name]
            // shave off 256 indices at the end to allow space for packing these indices into enums
            @max          [0xFFFF_FF00]
            @vis          [$v]
            @debug_format ["{}"]
                          $($tokens)+);
    );

    // ---- private rules ----

    // Base case, user-defined constants (if any) have already been defined
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]) => (
        $(#[$attrs])*
        #[derive(Copy, PartialEq, Eq, Hash, PartialOrd, Ord, $($derives),*)]
        $v struct $type {
            private: u32
        }

        impl Clone for $type {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl $type {
            /// Maximum value the index can take, as a `u32`.
            $v const MAX_AS_U32: u32 = $max;

            /// Maximum value the index can take.
            $v const MAX: Self = Self::from_u32($max);

            /// Creates a new index from a given `usize`.
            ///
            /// # Panics
            ///
            /// Will panic if `value` exceeds `MAX`.
            #[inline]
            $v const fn from_usize(value: usize) -> Self {
                assert!(value <= ($max as usize));
                // SAFETY: We just checked that `value <= max`.
                unsafe {
                    Self::from_u32_unchecked(value as u32)
                }
            }

            /// Creates a new index from a given `u32`.
            ///
            /// # Panics
            ///
            /// Will panic if `value` exceeds `MAX`.
            #[inline]
            $v const fn from_u32(value: u32) -> Self {
                assert!(value <= $max);
                // SAFETY: We just checked that `value <= max`.
                unsafe {
                    Self::from_u32_unchecked(value)
                }
            }

            /// Creates a new index from a given `u32`.
            ///
            /// # Safety
            ///
            /// The provided value must be less than or equal to the maximum value for the newtype.
            /// Providing a value outside this range is undefined due to layout restrictions.
            ///
            /// Prefer using `from_u32`.
            #[inline]
            $v const unsafe fn from_u32_unchecked(value: u32) -> Self {
                Self { private: value }
            }

            /// Extracts the value of this index as a `usize`.
            #[inline]
            $v const fn index(self) -> usize {
                self.as_usize()
            }

            /// Extracts the value of this index as a `u32`.
            #[inline]
            $v const fn as_u32(self) -> u32 {
                self.private
            }

            /// Extracts the value of this index as a `usize`.
            #[inline]
            $v const fn as_usize(self) -> usize {
                self.as_u32() as usize
            }
        }

        impl std::ops::Add<usize> for $type {
            type Output = Self;

            fn add(self, other: usize) -> Self {
                Self::from_usize(self.index() + other)
            }
        }

        impl $crate::index::Idx for $type {
            #[inline]
            fn new(value: usize) -> Self {
                Self::from_usize(value)
            }

            #[inline]
            fn index(self) -> usize {
                self.as_usize()
            }
        }


        impl From<$type> for u32 {
            #[inline]
            fn from(v: $type) -> u32 {
                v.as_u32()
            }
        }

        impl From<$type> for usize {
            #[inline]
            fn from(v: $type) -> usize {
                v.as_usize()
            }
        }

        impl From<usize> for $type {
            #[inline]
            fn from(value: usize) -> Self {
                Self::from_usize(value)
            }
        }

        impl From<u32> for $type {
            #[inline]
            fn from(value: u32) -> Self {
                Self::from_u32(value)
            }
        }

        $crate::newtype_index!(
            @handle_debug
            @derives      [$($derives,)*]
            @type         [$type]
            @debug_format [$debug_format]);
    );

    // base case for handle_debug where format is custom. No Debug implementation is emitted.
    (@handle_debug
     @derives      [$($_derives:ident,)*]
     @type         [$type:ident]
     @debug_format [custom]) => ();

    // base case for handle_debug, no debug overrides found, so use default
    (@handle_debug
     @derives      []
     @type         [$type:ident]
     @debug_format [$debug_format:tt]) => (
        impl ::std::fmt::Debug for $type {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(fmt, $debug_format, self.as_u32())
            }
        }
    );

    // It's not Debug, so just pop it off the front of the derives stack and check the rest.
    (@handle_debug
     @derives      [$_derive:ident, $($derives:ident,)*]
     @type         [$type:ident]
     @debug_format [$debug_format:tt]) => (
        $crate::newtype_index!(
            @handle_debug
            @derives      [$($derives,)*]
            @type         [$type]
            @debug_format [$debug_format]);
    );



    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      []
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );
}

impl Idx for usize {
    fn new(idx: usize) -> Self {
        idx
    }

    fn index(self) -> usize {
        self
    }
}

impl Idx for u32 {
    fn new(idx: usize) -> Self {
        idx as u32
    }

    fn index(self) -> usize {
        self as usize
    }
}
