mod tests;
use tests::generic_test;
use tests::trait_test;

fn main() {
    generic_test::test();
    println!("\n----------------------------------------------------------\n");
    trait_test::test();
}
