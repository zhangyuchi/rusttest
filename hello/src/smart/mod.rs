mod arctest;
mod listcons;
mod loopref;

pub fn run_test() {
    listcons::list_cons();
    loopref::loopref();
    arctest::thread_ref();
}
