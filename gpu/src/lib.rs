#![cfg_attr(
    target_os = "cuda",
    no_std,
    feature(register_attr),
    register_attr(nvvm_internal)
)]
//如果我们为 GPU 编译 crate (target_os = "cuda")，它仅应用属性。
//它将 crate 声明为 CUDA 目标上的 no_std。
//它注册了代码生成器所需的一个特殊属性，用于确定哪些函数是 GPU 内核之类的事情。
//它明确包含内核宏和线程

#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use cuda_std::*;
use cuda_std::GpuFloat;
use cuda_std::vek;
use cust_core::DeviceCopy;

pub type Vec3<T = f32> = vek::Vec3<T>;

#[derive(Default, Clone, Copy, DeviceCopy)]
#[repr(C)]
pub struct Add {
    pub a: Vec3,
    pub b: Vec3,
}

impl Add {
    pub fn display(&self) {
        cuda_std::println!("hello_world {}",self.a);
    }
}

#[kernel]
    pub unsafe fn add(a: &[f32], b: &[f32], c: *mut f32) {
        let idx = thread::index_1d() as usize;
        if idx < a.len() {
            let elem = &mut *c.add(idx);
            *elem = a[idx] + b[idx];
        }
    }