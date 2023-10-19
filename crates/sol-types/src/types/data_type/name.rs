use super::{Array, FixedArray};
use crate::SolType;
use core::{marker::PhantomData, mem::MaybeUninit};

struct Buffer<T: SolType>([u8; T::NAME.len()]);

impl<T: SolType> Array<T> {
    const STRS: &'static [&'static str] = &[T::NAME, "[]"];
    const LEN: usize = len(Self::STRS);
    const NAME_B: [u8; Self::LEN] = const_format::concatcp!(Self::STRS[0], Self::STRS[1]);
    pub(super) const NAME: &'static str = as_str(&Self::NAME_B);
}

impl<T: SolType, const N: usize> FixedArray<T, N> {
    pub(super) const NAME: &'static str = unsafe {
        let mut s = crate::impl_core::uninit_array::<u8, { u16::MAX as usize }>();

        let mut i = 0;
        let name = T::NAME.as_bytes();
        while i < name.len() {
            s[i] = MaybeUninit::new(name[i]);
            i += 1;
        }
        s[i] = MaybeUninit::new(b'[');
        i += 1;
        // TODO
        s[i] = MaybeUninit::new(b']');
        i += 1;

        as_str(core::slice::from_raw_parts(s.as_ptr().cast(), i))
    };
}

#[track_caller]
const fn as_str(sl: &[u8]) -> &str {
    match core::str::from_utf8(sl) {
        Ok(s) => s,
        Err(_) => panic!("invalid UTF-8"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sol_data;

    #[test]
    fn array() {
        assert_eq!(Array::<sol_data::Bool>::NAME, "bool[]");
    }
}
