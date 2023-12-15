mod ownership_test;
mod reference_test;

fn main() {
    ownership_test::test();
    println!("\n--------------------------------------------\n");
    reference_test::test();
}
