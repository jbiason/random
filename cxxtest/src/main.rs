#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxxtest/src/external/headers/aclass.hpp");

        type AClass;

        fn new_aclass(value: u32) -> UniquePtr<AClass>;
        fn process(&self);
    }
}

fn main() {
    println!("Hello, world!");
    let aclass = ffi::new_aclass(12);
    aclass.process();
}
