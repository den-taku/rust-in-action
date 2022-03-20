fn main() {
    let fn_ptr = noop as usize;
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x{fn_ptr:x}");
    println!("noop as *const T: {typed_fn_ptr:p}");
}

fn noop() {}
