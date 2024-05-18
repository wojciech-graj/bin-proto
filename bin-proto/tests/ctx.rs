use bin_proto::{Protocol, ByteOrder};

#[derive(Debug)]
struct Ctx(bool);

#[derive(Debug)]
struct CtxCheck;

impl Protocol for CtxCheck {
    fn read(
        _: &mut dyn bin_proto::BitRead,
        _: bin_proto::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<Self, bin_proto::Error> {
        ctx.downcast_mut::<Ctx>().unwrap().0 = true;
        Ok(Self)
    }

    fn write(
        &self,
        _: &mut dyn bin_proto::BitWrite,
        _: bin_proto::ByteOrder,
        ctx: &mut dyn std::any::Any,
    ) -> Result<(), bin_proto::Error> {
        ctx.downcast_mut::<Ctx>().unwrap().0 = true;
        Ok(())
    }
}

#[derive(Debug, Protocol)]
struct CtxCheckWrapper(CtxCheck);

#[test]
fn read_ctx_passed() {
    let mut ctx = Ctx(false);
    CtxCheck::from_bytes_ctx(&[], ByteOrder::BigEndian, &mut ctx).unwrap();
    assert!(ctx.0);
}

#[test]
fn write_ctx_passed() {
    let mut ctx = Ctx(false);
    CtxCheck.bytes_ctx(ByteOrder::BigEndian, &mut ctx).unwrap();
    assert!(ctx.0);
}

#[test]
fn read_ctx_passed_recur() {
    let mut ctx = Ctx(false);
    CtxCheckWrapper(CtxCheck)
        .bytes_ctx(ByteOrder::BigEndian, &mut ctx)
        .unwrap();
    assert!(ctx.0);
}

#[test]
fn write_ctx_passed_recur() {
    let mut ctx = Ctx(false);
    CtxCheckWrapper(CtxCheck)
        .bytes_ctx(ByteOrder::BigEndian, &mut ctx)
        .unwrap();
    assert!(ctx.0);
}
