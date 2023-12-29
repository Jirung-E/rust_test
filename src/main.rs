// mod tests;
// use tests::collection_test;
mod samples;
use samples::rectangles;

fn main() {
    println!("\n--------------------------------------------\n");
    rectangles::test();
}
