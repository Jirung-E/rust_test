// mod ownership_test;
// mod reference_test;
// mod slice_test;
// mod struct_test;
// mod enum_test;
// mod module_test;

mod samples;
use samples::restaurant;

fn main() {
    // ownership_test::test();
    // println!("\n--------------------------------------------\n");
    // reference_test::test();
    // println!("\n--------------------------------------------\n");
    // slice_test::test();
    // println!("\n--------------------------------------------\n");
    // struct_test::test();
    // println!("\n--------------------------------------------\n");
    // enum_test::test();
    // println!("\n--------------------------------------------\n");
    // module_test::test();
    println!("\n--------------------------------------------\n");
    restaurant::test();
}
