pub mod ownership_test;
pub mod reference_test;
pub mod slice_test;
pub mod struct_test;
pub mod enum_test;
pub mod module_test;
pub mod collection_test;

pub fn test_all() {
    ownership_test::test();
    println!("\n--------------------------------------------\n");
    reference_test::test();
    println!("\n--------------------------------------------\n");
    slice_test::test();
    println!("\n--------------------------------------------\n");
    struct_test::test();
    println!("\n--------------------------------------------\n");
    enum_test::test();
    println!("\n--------------------------------------------\n");
    module_test::test();
    println!("\n--------------------------------------------\n");
    collection_test::test();
}