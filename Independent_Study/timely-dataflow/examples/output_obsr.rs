extern crate timely;

use timely::dataflow::operators::{ToStream, Inspect};

fn main() {
    timely::execute_from_args(std::env::args(), |worker| {
        worker.dataflow::<(),_,_>(|scope| {
            (0 .. 9)
                .to_stream(scope)
                .inspect(|x| println!("hello: {}", x));
        });
    }).unwrap();
}