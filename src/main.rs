mod utilities;
mod package;
mod truck;

fn main() {
    utilities::set_creation::create_test_set(10, 100, 500, Some(10));
}
