use super::Executor;

mod aload;
mod anewarray;
mod dup;
mod get_static;
mod iconst;
mod invoke_special;
mod invoke_static;
mod invoke_virtual;
mod ldc;
mod new;
mod ret;

const ICONST_0: u8 = 0x3;
const LDC: u8 = 0x12;
const RET: u8 = 0xb1;
const GETSTATIC: u8 = 0xb2;
const INVOKEVIRTUAL: u8 = 0xb6;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const NEW: u8 = 0xbb;
const DUP: u8 = 0x59;
const ALOAD_0: u8 = 0x2a;
const ANEWARRAY: u8 = 0xbd;

type OpMethod = fn(&mut Executor);

pub fn get_op(op_code: &u8) -> Option<OpMethod> {
    match *op_code {
        INVOKESTATIC => Some(invoke_static::perform as OpMethod),
        GETSTATIC => Some(get_static::perform as OpMethod),
        INVOKEVIRTUAL => Some(invoke_virtual::perform as OpMethod),
        INVOKESPECIAL => Some(invoke_special::perform as OpMethod),
        NEW => Some(new::perform as OpMethod),
        DUP => Some(dup::perform as OpMethod),
        ALOAD_0 => Some(aload::aload_0 as OpMethod),
        RET => Some(ret::perform as OpMethod),
        LDC => Some(ldc::perform as OpMethod),
        ICONST_0 => Some(iconst::iconst_0 as OpMethod),
        ANEWARRAY => Some(anewarray::perform as OpMethod),
        _ => None,
    }
}
