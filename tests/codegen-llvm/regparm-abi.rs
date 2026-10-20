// Checks that `extern "regparmN"` marks arguments `inreg` the way GCC's
// `__attribute__((regparm(N)))` passes them in EAX, EDX, ECX.

//@ add-minicore
//@ compile-flags: --target i686-unknown-linux-gnu -Cno-prepopulate-passes -Copt-level=3
//@ needs-llvm-components: x86

#![crate_type = "lib"]
#![no_core]
#![feature(no_core, lang_items)]

extern crate minicore;

#[repr(C)]
pub struct Pair {
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct Six {
    a: u16,
    b: u16,
    c: u16,
}

#[repr(C)]
pub struct Big {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

#[repr(C)]
pub struct Ret3 {
    a: i32,
    b: i32,
    c: i32,
}

#[repr(C)]
pub struct OneFloat {
    x: f32,
}

// CHECK: @ints1(i32 inreg noundef %_1, i32 noundef %_2)
#[no_mangle]
pub extern "regparm1" fn ints1(_: i32, _: i32) {}

// CHECK: @ints2(i32 inreg noundef %_1, ptr inreg noundef %_2, i32 noundef %_3)
#[no_mangle]
pub extern "regparm2" fn ints2(_: i32, _: *const u8, _: i32) {}

// CHECK: @ints3(i8 inreg noundef signext %_1, i32 inreg noundef %_2, i32 inreg noundef %_3,
// CHECK-SAME: i32 noundef %_4)
#[no_mangle]
pub extern "regparm3" fn ints3(_: i8, _: i32, _: i32, _: i32) {}

// a 64-bit int takes two registers (EAX:EDX)
// CHECK: @wide3(i64 inreg noundef %_1, i32 inreg noundef %_2)
#[no_mangle]
pub extern "regparm3" fn wide3(_: i64, _: i32) {}

// it can't be split between a register and the stack, and nothing after it gets a register
// CHECK: @wide1(i64 noundef %_1, i32 noundef %_2)
#[no_mangle]
pub extern "regparm1" fn wide1(_: i64, _: i32) {}

// the hidden return pointer takes the first register
// CHECK: @ret3(ptr {{.*}}inreg{{.*}}sret{{.*}}%_0, i32 inreg noundef %a, i32 inreg noundef %b,
// CHECK-SAME: i32 noundef %c)
#[no_mangle]
pub extern "regparm3" fn ret3(a: i32, b: i32, c: i32) -> Ret3 {
    Ret3 { a, b, c }
}

// CHECK: @pair([2 x i32] inreg %0, i32 inreg noundef %_2)
#[no_mangle]
pub extern "regparm3" fn pair(_: Pair, _: i32) {}

// rounded up to whole registers, like GCC
// CHECK: @six([2 x i32] inreg %0, i32 inreg noundef %_2)
#[no_mangle]
pub extern "regparm3" fn six(_: Six, _: i32) {}

// a struct that doesn't fit stays on the stack and uses up the rest of the registers
// CHECK: @big(i32 inreg noundef %_1, ptr {{.*}}byval{{.*}}%_2, i32 noundef %_3)
#[no_mangle]
pub extern "regparm3" fn big(_: i32, _: Big, _: i32) {}

// floats don't use a register slot
// CHECK: @mixed(i32 inreg noundef %_1, double noundef %_2, i32 inreg noundef %_3,
// CHECK-SAME: i32 inreg noundef %_4)
#[no_mangle]
pub extern "regparm3" fn mixed(_: i32, _: f64, _: i32, _: i32) {}

// neither do structs wrapping a single float
// CHECK: @one_float(ptr {{.*}}byval{{.*}}%_1, i32 inreg noundef %_2)
#[no_mangle]
pub extern "regparm1" fn one_float(_: OneFloat, _: i32) {}
