extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::ToStream;

fn main() {
    // initializes and runs a timely dataflow.
    timely::execute_from_args(std::env::args(), |worker| {

        let mut input = InputHandle::<(), String>::new();

        // define a new dataflow
        worker.dataflow(|scope| {

            //let stream1 = input.to_stream(scope);
            let stream2 = (0 .. 9).to_stream(scope);

        });

    }).unwrap();
}