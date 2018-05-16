extern crate communicator;

use communicator::TrafficLight::{Red, Yellow};

fn main() {
    communicator::client::connect();

    let red = Red;
    let yellow = Yellow;
    let green = communicator::TrafficLight::Green;


    use communicator::TrafficLight::*;
    let green = Green;

}