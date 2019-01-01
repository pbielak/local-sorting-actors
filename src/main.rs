extern crate actix;
extern crate futures;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate rand;
extern crate time;

use actix::prelude::*;
use futures::{future, Future};

mod messages;
mod supervisor;
mod sorting_actor;
mod util;


fn run_default() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    const P: usize = 90;//9_000_000; // Array size
    const N: usize = 5; // Number of actors
    const K: usize = 3; // Number of chunks

    let numbers = util::random_vec(P);
    let sort_req = messages::SortingRequest::new(numbers);


    System::run(|| {
        let supervisor = Arbiter::start(|_| supervisor::SupervisorActor::new("MASTER", N, K));
        let res = supervisor.send(sort_req);

        Arbiter::spawn(
            res.then(|r| {
                match r {
                    Ok(r) => info!("SortingResponse[Vec[{:?}], Duration: {} (ms)]", r.values.len(), r.duration),
                    _ => error!("Error occurred!")
                }

                System::current().stop();
                future::result(Ok(()))
            })
        );
    });
}


fn get_time(numbers: &Vec<i64>, n: usize, k: usize) -> i64 {
    let _sys = System::new("Sorting system");
    let sort_req = messages::SortingRequest::new(numbers.clone());

    let supervisor = Arbiter::start(move |_| supervisor::SupervisorActor::new("MASTER", n, k));
    let res = supervisor.send(sort_req);

    res.wait().unwrap().duration
}


fn measure_times() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    const P: usize = 900_000; // Array size
    let numbers = util::random_vec(P);

    const MAX_N: usize = 10;
    const MAX_K: usize = 10;

    let mut results: Vec<(usize, usize, i64)> = vec![];

    for n in 1..=MAX_N {
        for k in 1..=MAX_K {
            info!("N: {} K: {}", n, k);
            results.push((n, k, get_time(&numbers, n, k)));
        }
    }

    info!("Results: {:?}", results);
}


fn main() {
    run_default()
    //measure_times()
}
