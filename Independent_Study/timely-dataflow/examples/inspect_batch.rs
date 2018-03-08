extern crate timely;

use timely::dataflow::operators::{ToStream, Inspect};

fn main() {
    timely::execute_from_args(std::env::args(), |worker| {
        worker.dataflow::<(),_,_>(|scope| {
            (0 .. 10)
                .to_stream(scope)
                .inspect_batch(|t, xs| println!("hello: {:?} @ {:?}", xs, t));
        });
    }).unwrap();
}