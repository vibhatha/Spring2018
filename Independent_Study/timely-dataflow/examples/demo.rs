extern crate timely;

use timely::dataflow::operators::*;

fn main() {
    timely::example(|scope| {

        // Produce all numbers less than each input number.
        (1 .. 10)
            .to_stream(scope)
            .flat_map(|x| (0 .. x));

    });
}