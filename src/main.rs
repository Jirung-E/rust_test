mod samples;
use samples::restaurant;
use samples::pyramid;

fn main() {
    println!("\n--------------------------------------------\n");
    restaurant::test();
    println!("\n--------------------------------------------\n");
    pyramid::test();
}
