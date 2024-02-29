mod box_test;
mod drop_test;
mod rc_test;
mod refcell_test;


pub fn test() {
    println!(" [ pointer test ] ");

    refcell_test::test();
}