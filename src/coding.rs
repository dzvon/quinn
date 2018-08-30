use bytes::{Buf, BufMut};

use varint;

#[derive(Fail, Debug, Copy, Clone, Eq, PartialEq)]
#[fail(display = "unexpected end of buffer")]
pub struct UnexpectedEnd;

pub type Result<T> = ::std::result::Result<T, UnexpectedEnd>;

pub trait Value: Sized {
    fn decode<B: Buf>(buf: &mut B) -> Result<Self>;
    fn encode<B: BufMut>(&self, buf: &mut B);
}

impl Value for u8 {
    fn decode<B: Buf>(buf: &mut B) -> Result<u8> {
        if buf.remaining() < 1 {
            return Err(UnexpectedEnd);
        }
        Ok(buf.get_u8())
    }
    fn encode<B: BufMut>(&self, buf: &mut B) {
        buf.put_u8(*self);
    }
}

impl Value for u16 {
    fn decode<B: Buf>(buf: &mut B) -> Result<u16> {
        if buf.remaining() < 2 {
            return Err(UnexpectedEnd);
        }
        Ok(buf.get_u16_be())
    }
    fn encode<B: BufMut>(&self, buf: &mut B) {
        buf.put_u16_be(*self);
    }
}

impl Value for u32 {
    fn decode<B: Buf>(buf: &mut B) -> Result<u32> {
        if buf.remaining() < 4 {
            return Err(UnexpectedEnd);
        }
        Ok(buf.get_u32_be())
    }
    fn encode<B: BufMut>(&self, buf: &mut B) {
        buf.put_u32_be(*self);
    }
}

impl Value for u64 {
    fn decode<B: Buf>(buf: &mut B) -> Result<u64> {
        if buf.remaining() < 8 {
            return Err(UnexpectedEnd);
        }
        Ok(buf.get_u64_be())
    }
    fn encode<B: BufMut>(&self, buf: &mut B) {
        buf.put_u64_be(*self);
    }
}

pub trait BufExt {
    fn get<T: Value>(&mut self) -> Result<T>;
    fn get_var(&mut self) -> Result<u64>;
}

impl<T: Buf> BufExt for T {
    fn get<U: Value>(&mut self) -> Result<U> {
        U::decode(self)
    }

    fn get_var(&mut self) -> Result<u64> {
        varint::read(self).ok_or(UnexpectedEnd)
    }
}

pub trait BufMutExt {
    fn write<T: Value>(&mut self, x: T);
    fn write_var(&mut self, x: u64);
}

impl<T: BufMut> BufMutExt for T {
    fn write<U: Value>(&mut self, x: U) {
        x.encode(self);
    }

    fn write_var(&mut self, x: u64) {
        varint::write(x, self).unwrap()
    }
}
