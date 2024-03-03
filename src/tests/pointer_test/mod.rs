mod box_test;
mod drop_test;
mod rc_test;
mod refcell_test;
mod reference_cycles_test;
mod weak_test;


pub fn test() {
    println!(" [ pointer test ] ");

    weak_test::test();
}