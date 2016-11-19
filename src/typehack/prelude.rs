pub use typehack::binary::{Nat, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14,
                           B15, B16, B17, B18, B19, B20, B21, B22, B23, B24, B25, B26, B27, B28,
                           B29, B30, B31, B32, B64, B128, B256, B512};
pub use typehack::data::{Data, DataVec};
pub use typehack::dim::{Dim, DimAdd, DimMul, DimShl};


pub type Sum<M: Dim, N: Dim> = <M as DimAdd<N>>::Result;
pub type Product<M: Dim, N: Dim> = <M as DimMul<N>>::Result;
pub type ShiftLeft<M: Dim, N: Dim> = <M as DimShl<N>>::Result;
