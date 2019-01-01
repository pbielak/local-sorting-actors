extern crate actix;
extern crate futures;
extern crate rand;
extern crate time;

use actix::prelude::*;
use futures::{future, Future};

mod msgs;
mod supervisor;
mod sorting_actor;
mod util;

use crate::supervisor::SupervisorActor;


fn run_default() {
    const P: usize = 90_000; // Array size
    const N: usize = 5; // Number of actors
    const K: usize = 5; // Number of chunks

    let numbers = util::random_vec(P);
    let sort_req = msgs::SortingRequest::new(numbers);


    System::run(|| {
        let supervisor = Arbiter::start(|_| SupervisorActor::new("MASTER", N, K));
        let res = supervisor.send(sort_req);

        Arbiter::spawn(
            res.then(|r| {
                match r {
                    Ok(r) => println!("Result: Sorted(Vec[{:?}], Duration: {} (ms)]", r.values.len(), r.duration),
                    _ => println!("Error occurred!")
                }

                System::current().stop();
                future::result(Ok(()))
            })
        );
    });
}


fn get_time(numbers: &Vec<i64>, n: usize, k: usize) -> i64 {
    let sort_req = msgs::SortingRequest::new(numbers.clone());

    let supervisor = Arbiter::start(move |_| SupervisorActor::new("MASTER", n, k));
    let res = supervisor.send(sort_req);

    res.wait().unwrap().duration
}


fn main() {
//    const P: usize = 900_000; // Array size
//    let numbers = util::random_vec(P);
//
//    const MAX_N: usize = 10;
//    const MAX_K: usize = 10;
//
//    let mut results: Vec<(usize, usize, i64)> = vec![];
//
//    for n in 1..=MAX_N {
//        for k in 1..=MAX_K {
//            results.push((n, k, get_time(&numbers, n, k)))
//        }
//    }
//
//    println!("Results: {:?}", results);

    run_default()
}
