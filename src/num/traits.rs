pub trait Float {
    fn sqrt(self) -> Self;
    fn recip(self) -> Self;
}


macro_rules! impl_float {
    ($($t:ident),*) => {
        $(impl Float for $t {
            #[inline]
            fn sqrt(self) -> $t {
                self.sqrt()
            }

            #[inline]
            fn recip(self) -> $t {
                self.recip()
            }
        })*
    }
}

impl_float!(f32, f64);


pub trait Zero {
    fn zero() -> Self;
}


macro_rules! impl_zero {
    ($($t:ident),* => $z:expr) => {
        $(impl Zero for $t {
            fn zero() -> Self { $z }
        })*
    }
}

impl_zero!(u8, u16, u32, u64, i8, i16, i32, i64 => 0);
impl_zero!(f32, f64 => 0.0);


pub trait One {
    fn one() -> Self;
}


macro_rules! impl_one {
    ($($t:ident),* => $o:expr) => {
        $(impl One for $t {
            fn one() -> Self { $o }
        })*
    }
}

impl_one!(u8, u16, u32, u64, i8, i16, i32, i64 => 1);
impl_one!(f32, f64 => 1.0);


pub trait ZeroByteEquivalent: Zero {}

macro_rules! impl_zbe {
    ($($t:ident),*) => {
        $(impl ZeroByteEquivalent for $t {})*
    }
}

impl_zbe!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
