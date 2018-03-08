#![allow(unused_variables)]
extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::{Input, Exchange, Inspect, Probe};

fn main() {
    // initializes and runs a timely dataflow.
    timely::execute_from_args(std::env::args(), |worker| {

        let index = worker.index();
        let mut input = InputHandle::new();

        // create a new input, exchange data, and inspect its output
        let probe = worker.dataflow(|scope|
            scope.input_from(&mut input)
                 .exchange(|x| *x)
                 .inspect(|x| {
                        // we only need to test factors up to sqrt(x) 
                        let limit = (*x as f64).sqrt() as u64;
                            if *x > 1 && (2 .. limit + 1).all(|i| x % i > 0) {
                            println!("{} is prime", x);
                }
})
                 .probe()
        );

        // introduce data and watch!
        for round in 0..100000 {
            if worker.index() == 0 {
                input.send(round);
            }
            input.advance_to(round + 1);
            // worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}