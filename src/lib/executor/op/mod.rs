use super::Executor;
use anyhow::{bail, Result};

mod aload;
mod anewarray;
mod dup;
mod get_static;
mod iconst;
mod ifne;
mod invoke_special;
mod invoke_static;
mod invoke_virtual;
mod ldc;
mod new;
mod putstatic;
mod ret;

const ICONST_0: u8 = 0x3;
const LDC: u8 = 0x12;
const RETURN: u8 = 0xb1;
const GETSTATIC: u8 = 0xb2;
const INVOKEVIRTUAL: u8 = 0xb6;
const INVOKESPECIAL: u8 = 0xb7;
const INVOKESTATIC: u8 = 0xb8;
const NEW: u8 = 0xbb;
const DUP: u8 = 0x59;
const ALOAD_0: u8 = 0x2a;
const ANEWARRAY: u8 = 0xbd;
const PUTSTATIC: u8 = 0xb3;
const IFNE: u8 = 0x9a;

type OpMethod = fn(&mut Executor) -> Result<()>;

pub fn get_op(op_code: u8) -> Result<OpMethod> {
    match op_code {
        INVOKESTATIC => Ok(invoke_static::perform as OpMethod),
        GETSTATIC => Ok(get_static::perform as OpMethod),
        INVOKEVIRTUAL => Ok(invoke_virtual::perform as OpMethod),
        INVOKESPECIAL => Ok(invoke_special::perform as OpMethod),
        NEW => Ok(new::perform as OpMethod),
        DUP => Ok(dup::perform as OpMethod),
        ALOAD_0 => Ok(aload::aload_0 as OpMethod),
        RETURN => Ok(ret::perform as OpMethod),
        LDC => Ok(ldc::perform as OpMethod),
        ICONST_0 => Ok(iconst::iconst_0 as OpMethod),
        ANEWARRAY => Ok(anewarray::perform as OpMethod),
        PUTSTATIC => Ok(putstatic::perform as OpMethod),
        IFNE => Ok(ifne::perform as OpMethod),
        _ => bail!("unknown op 0x{op_code:X}"),
    }
}

pub fn name(op_code: u8) -> Result<String> {
    match op_code {
        INVOKESTATIC => Ok("invokestatic".to_string()),
        GETSTATIC => Ok("getstatic".to_string()),
        INVOKEVIRTUAL => Ok("invokevirtual".to_string()),
        INVOKESPECIAL => Ok("invokespecial".to_string()),
        NEW => Ok("new".to_string()),
        DUP => Ok("dup".to_string()),
        ALOAD_0 => Ok("aload_0".to_string()),
        RETURN => Ok("ret".to_string()),
        LDC => Ok("ldc".to_string()),
        ICONST_0 => Ok("iconst_0".to_string()),
        ANEWARRAY => Ok("anewarray".to_string()),
        PUTSTATIC => Ok("putstatic".to_string()),
        IFNE => Ok("ifne".to_string()),
        _ => bail!("unknown op 0x{op_code:X}"),
    }
}

pub fn is_return(op_code: u8) -> bool {
    match op_code {
        RETURN => true,
        _ => false,
    }
}
