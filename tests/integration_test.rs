extern crate communicator;

mod common;


#[test]
fn it_adds_two_ingrt() {
    common::setup();
    assert_eq!(4, communicator::add_two(2));
}