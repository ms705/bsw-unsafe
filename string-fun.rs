use std::slice;

struct S1 { ptr: *mut u8, len: usize }

impl S1 {
    pub fn from(p: *mut u8, l: usize) -> S1 {
        S1 { ptr: p, len: l }
    }
    pub fn set_len(&mut self, l: usize) {
        self.len = l;
    }
    pub fn get(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let s1 = S1::from(String::from("hello").as_mut_ptr(), 6);

    // print string as hexadecimal characters
    println!("{:x?}", s1.get())
}
