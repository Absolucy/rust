// Checks the actual registers `extern "regparmN"` uses, which must match GCC's
// `__attribute__((regparm(N)))`: EAX, EDX, ECX, caller cleanup.

//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: -Copt-level=3 --target=i686-unknown-linux-gnu -Crelocation-model=static
//@ needs-llvm-components: x86

#![feature(no_core)]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;

#[repr(C)]
pub struct Pair {
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct Ret3 {
    a: i32,
    b: i32,
    c: i32,
}

extern "regparm3" {
    fn ext_wide(v: u64, w: u32);
    fn ext_ret(a: i32, b: i32, c: i32) -> Ret3;
}

// CHECK-LABEL: def_store:
// CHECK: movl %edx, (%eax)
// CHECK-NEXT: retl
#[no_mangle]
pub unsafe extern "regparm2" fn def_store(p: *mut i32, v: i32) {
    *p = v;
}

// CHECK-LABEL: def_wide:
// CHECK-DAG: movl %edx, 4(%ecx)
// CHECK-DAG: movl %eax, (%ecx)
// CHECK: retl
#[no_mangle]
pub unsafe extern "regparm3" fn def_wide(v: u64, p: *mut u64) {
    *p = v;
}

// CHECK-LABEL: def_pair:
// CHECK-DAG: movl %edx, 4(%ecx)
// CHECK-DAG: movl %eax, (%ecx)
// CHECK: retl
#[no_mangle]
pub unsafe extern "regparm3" fn def_pair(v: Pair, p: *mut Pair) {
    *p = v;
}

// the return pointer arrives in EAX and the callee must not pop it (no `retl $4`)
// CHECK-LABEL: def_ret:
// CHECK-DAG: movl %edx, (%eax)
// CHECK-DAG: movl %ecx, 4(%eax)
// CHECK: retl{{$}}
#[no_mangle]
pub extern "regparm3" fn def_ret(a: i32, b: i32, c: i32) -> Ret3 {
    Ret3 { a, b, c }
}

// CHECK-LABEL: call_wide:
// CHECK-DAG: movl $1, %eax
// CHECK-DAG: movl $2, %edx
// CHECK-DAG: movl $3, %ecx
// CHECK: {{calll|jmp}} ext_wide
#[no_mangle]
pub unsafe fn call_wide() {
    // low word 1, high word 2
    ext_wide(0x2_0000_0001, 3);
}

// CHECK-LABEL: call_ret:
// CHECK-DAG: movl $1, %edx
// CHECK-DAG: movl $2, %ecx
// CHECK-DAG: leal {{.*}}, %eax
// CHECK: calll ext_ret
#[no_mangle]
pub unsafe fn call_ret() -> i32 {
    ext_ret(1, 2, 3).c
}
