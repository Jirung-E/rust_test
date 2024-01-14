mod samples;
mod tests;
// use samples::iot as module;
use tests::collection_test as module;

fn main() {
    module::test();
}
