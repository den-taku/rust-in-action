// use std::borrow::Cow;
// use std::ffi::CStr;
use std::mem::{drop, size_of};
// use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (unsigned int):");
    println!(" place: {:p}", &a);
    println!(" size: {:?} bytes", size_of::<usize>());
    println!(" value: {a:?}");

    println!("b (reference to B):");
    println!(" place: {:p}", &b);
    println!(" size: {:?} bytes", size_of::<&[u8; 10]>());
    println!(" referent: {b:p}");

    println!("c (Boxed C):");
    println!(" place: {:p}", &c);
    println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
    println!(" referent: {c:p}");

    println!("B (array of 10 Bytes):");
    println!(" place: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {B:?}");

    println!("C (array of 11 Bytes):");
    println!(" place: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {C:?}");

    // let a = 42;
    // let b: String;
    // let c: Cow<str>;

    // unsafe {
    //     let b_ptr = &B as *const u8 as *mut u8;
    //     b = String::from_raw_parts(b_ptr, 10, 10);
    //     let c_ptr = &C as *const u8 as *const c_char;
    //     c = CStr::from_ptr(c_ptr).to_string_lossy();
    // }

    // println!("a: {a}, b: {b}, c: {c}");

    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {a} ({a_ptr:p}...0x{:x})", a_addr + 7);

    let ptr = 42 as *const Vec<String>;
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{ptr:p} -> {new_addr:p}");
        // println!("{:?}", *new_addr);
    }

    let pw = "justok";
    let is_strong = is_strong(pw);
    println!("{is_strong}");

    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);

    println!("{a} + {b} = {}", a + *b);

    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);
    let result1 = *a + *b + *c;
    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{result1} {result2}");
}

// fn is_strong<T: AsRef<str>>(password: T) -> bool {
//     password.as_ref().len() > 5
// }

fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}
