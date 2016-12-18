extern crate jni_sys;

/// This crate allows to call Java code from Rust via JNI.

mod java;
mod jvm;
mod jvm_class;
mod jvm_method;
mod jvm_object;

pub use jvm::{jvalue_from_jboolean, Jvm};
pub use jvm_class::JvmClass;
pub use jvm_method::JvmMethod;
pub use jvm_object::JvmObject;
