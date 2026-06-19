fn main() {
    let mut s = String::from("hello");
    let p: *mut String = &mut s;

    drop(s);

    unsafe {
        // use after free!
        println!("{}", *p);
    }
}
