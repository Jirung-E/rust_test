mod ownership_test;
mod reference_test;
mod slice_test;
mod struct_test;
mod enum_test;

fn main() {
    ownership_test::test();
    println!("\n--------------------------------------------\n");
    reference_test::test();
    println!("\n--------------------------------------------\n");
    slice_test::test();
    println!("\n--------------------------------------------\n");
    struct_test::test();
    println!("\n--------------------------------------------\n");
    enum_test::test();
}
