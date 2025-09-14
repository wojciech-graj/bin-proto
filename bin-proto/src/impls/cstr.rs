use core::ffi::CStr;

use bitstream_io::{BitWrite, Endianness};

use crate::{util, BitEncode, Result};

impl<Ctx> BitEncode<Ctx> for CStr {
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, (): ()) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        util::encode_items::<_, E, _, _>(self.to_bytes_with_nul().iter(), write, ctx)
    }
}

#[cfg(feature = "alloc")]
#[allow(clippy::wildcard_imports)]
mod decode {
    use alloc::{boxed::Box, ffi::CString};
    use bitstream_io::BitRead;

    use crate::BitDecode;

    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<Ctx> BitDecode<Ctx> for Box<CStr> {
        fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: ()) -> Result<Self>
        where
            R: BitRead,
            E: Endianness,
        {
            CString::decode::<_, E>(read, ctx, tag).map(Into::into)
        }
    }

    test_decode!(Box<CStr>; [0x41, 0x42, 0x43, 0] => CString::new("ABC").unwrap().into());
}

test_encode!(
    &CStr; CStr::from_bytes_with_nul(&[0x41, 0x42, 0x43, 0]).unwrap() => [0x41, 0x42, 0x43, 0]
);
