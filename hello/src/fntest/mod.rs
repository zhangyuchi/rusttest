use crate::fntest::fnmuttest::fnmul_test;

mod fnmuttest;
mod fnoncetest;
mod fntest;

pub fn fn_test() {
    fnoncetest::fn_once_test();
    println!("--------------------------------");
    fntest::fn_test();
    println!("--------------------------------");
    fnmuttest::fnmul_test();
}
