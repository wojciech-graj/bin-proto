#![cfg(feature = "derive")]

use std::marker::PhantomData;

use bin_proto::{BitDecode, BitEncode, ByteOrder};

trait Boolean {
    fn set(&mut self);
}

impl Boolean for bool {
    fn set(&mut self) {
        *self = true
    }
}

trait TraitWithGeneric<'a, T>
where
    T: Boolean,
{
}

trait CtxTrait {
    fn call(&mut self);
}

#[derive(Debug)]
struct CtxStruct(bool);

impl CtxTrait for CtxStruct {
    fn call(&mut self) {
        self.0 = true
    }
}

#[derive(Debug)]
struct CtxStructWithGenerics<'a, T>(&'a mut T);

impl<'a, T> CtxTrait for CtxStructWithGenerics<'a, T>
where
    T: Boolean,
{
    fn call(&mut self) {
        self.0.set()
    }
}

#[derive(Debug)]
struct CtxCheck;

impl<Ctx: CtxTrait> BitDecode<Ctx> for CtxCheck {
    fn decode(
        _: &mut dyn bin_proto::BitRead,
        _: bin_proto::ByteOrder,
        ctx: &mut Ctx,
        _: (),
    ) -> Result<Self, bin_proto::Error> {
        ctx.call();
        Ok(Self)
    }
}

impl<Ctx: CtxTrait> BitEncode<Ctx> for CtxCheck {
    fn encode(
        &self,
        _: &mut dyn bin_proto::BitWrite,
        _: bin_proto::ByteOrder,
        ctx: &mut Ctx,
        (): (),
    ) -> Result<(), bin_proto::Error> {
        ctx.call();
        Ok(())
    }
}

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx = CtxStruct)]
struct CtxCheckStructWrapper(CtxCheck);

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx = CtxStructWithGenerics<'a, bool>, ctx_generics('a))]
struct CtxCheckStructWrapperWithGenericsConcreteBool(CtxCheck);

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx = CtxStructWithGenerics<'a, T>, ctx_generics('a, T: Boolean))]
struct CtxCheckStructWrapperWithGenerics(CtxCheck);

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx_bounds(TraitWithGeneric<'a, bool>, CtxTrait), ctx_generics('a))]
struct CtxCheckBoundsWithGenericsConcreteBool(CtxCheck);

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx_bounds(TraitWithGeneric<'a, T>, CtxTrait), ctx_generics('a))]
struct CtxCheckBoundsWithGenerics<T: Boolean>(CtxCheck, PhantomData<T>);

#[derive(Debug, BitDecode, BitEncode)]
#[codec(ctx_bounds(CtxTrait))]
struct CtxCheckTraitWrapper(CtxCheck);

#[test]
fn decode_ctx_passed() {
    let mut ctx = CtxStruct(false);
    CtxCheck::decode_bytes_ctx(&[], ByteOrder::BigEndian, &mut ctx, ()).unwrap();
    assert!(ctx.0);
}

#[test]
fn encode_ctx_passed() {
    let mut ctx = CtxStruct(false);
    CtxCheck
        .encode_bytes_ctx(ByteOrder::BigEndian, &mut ctx, ())
        .unwrap();
    assert!(ctx.0);
}

#[test]
fn decode_ctx_passed_recur_struct() {
    let mut ctx = CtxStruct(false);
    CtxCheckStructWrapper(CtxCheck)
        .encode_bytes_ctx(ByteOrder::BigEndian, &mut ctx, ())
        .unwrap();
    assert!(ctx.0);
}

#[test]
fn encode_ctx_passed_recur_struct() {
    let mut ctx = CtxStruct(false);
    CtxCheckStructWrapper(CtxCheck)
        .encode_bytes_ctx(ByteOrder::BigEndian, &mut ctx, ())
        .unwrap();
    assert!(ctx.0);
}

#[test]
fn decode_ctx_passed_recur_trait() {
    let mut ctx = CtxStruct(false);
    CtxCheckTraitWrapper(CtxCheck)
        .encode_bytes_ctx(ByteOrder::BigEndian, &mut ctx, ())
        .unwrap();
    assert!(ctx.0);
}

#[test]
fn encode_ctx_passed_recur_trait() {
    let mut ctx = CtxStruct(false);
    CtxCheckTraitWrapper(CtxCheck)
        .encode_bytes_ctx(ByteOrder::BigEndian, &mut ctx, ())
        .unwrap();
    assert!(ctx.0);
}
