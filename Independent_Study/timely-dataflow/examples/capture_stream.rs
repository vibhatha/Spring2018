extern crate timely;

use timely::dataflow::operators::{ToStream, Capture};
use timely::dataflow::operators::capture::Extract;

fn main() {
    let (data1, data2) = timely::example(|scope| {
        let data1 = (0..3).to_stream(scope).capture();
        let data2 = vec![0,1,2].to_stream(scope).capture();
        (data1, data2)
    });

    assert_eq!(data1.extract(), data2.extract());
}