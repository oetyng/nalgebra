use na::{Scalar, RealField, U3, DefaultAllocator};

use crate::traits::{Number, Alloc, Dimension};
use crate::aliases::TVec;

pub fn bitCount<T>(v: T) -> i32 {
    unimplemented!()
}

pub fn bitCount2<N: Scalar + Copy, D: Dimension>(v: &TVec<N, D>) -> TVec<i32, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn bitfieldExtract<N: Scalar + Copy, D: Dimension>(Value: &TVec<N, D>, Offset: i32, Bits: i32) -> TVec<N, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn bitfieldInsert<N: Scalar + Copy, D: Dimension>(Base: &TVec<N, D>, Insert: &TVec<N, D>, Offset: i32, Bits: i32) -> TVec<N, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn bitfieldReverse<N: Scalar + Copy, D: Dimension>(v: &TVec<N, D>) -> TVec<N, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn findLSB<IU>(x: IU) -> u32 {
    unimplemented!()
}

pub fn findLSB2<N: Scalar + Copy, D: Dimension>(v: &TVec<N, D>) -> TVec<i32, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn findMSB<IU>(x: IU) -> i32 {
    unimplemented!()
}

pub fn findMSB2<N: Scalar + Copy, D: Dimension>(v: &TVec<N, D>) -> TVec<i32, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn imulExtended<N: Scalar + Copy, D: Dimension>(x: &TVec<i32, D>, y: &TVec<i32, D>, msb: &TVec<i32, D>, lsb: &TVec<i32, D>)
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn uaddCarry<N: Scalar + Copy, D: Dimension>(x: &TVec<u32, D>, y: &TVec<u32, D>, carry: &TVec<u32, D>) -> TVec<u32, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}

pub fn umulExtended<N: Scalar + Copy, D: Dimension>(x: &TVec<u32, D>, y: &TVec<u32, D>, msb: &TVec<u32, D>, lsb: &TVec<u32, D>)
    where DefaultAllocator: Alloc<N, D> {
unimplemented!()
}

pub fn usubBorrow<N: Scalar + Copy, D: Dimension>(x: &TVec<u32, D>, y: &TVec<u32, D>, borrow: &TVec<u32, D>) -> TVec<u32, D>
    where DefaultAllocator: Alloc<N, D> {
    unimplemented!()
}
