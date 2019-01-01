extern crate actix;
extern crate futures;
extern crate rand;
extern crate time;

use actix::prelude::*;
use futures::Future;


mod msgs;
mod supervisor;
mod sorting_actor;
mod util;

use crate::supervisor::SupervisorActor;


fn main() {
//    let system = System::new("Sorting system");
//
//    const P: usize = 10; // Array size
//    const N: usize = 2; // Number of actors
//    const K: usize = 5; // Number of chunks
//
//    let supervisor = SupervisorActor::new("MASTER", N, K).start();
//
//    let numbers = util::random_vec(P);
//    let sort_req = msgs::SortingRequest::new(numbers, None);
//
//    let res = supervisor.send(sort_req);
//
//    Arbiter::spawn(
//        res
//            .map(|r| {
//                println!("Result: Sorted(Vec[{}], Duration: {} (ms)]", r.values.len(), r.duration);
//                System::current().stop();
//            })
//            .map_err(|_| {
//                println!("Error occurred!");
//                System::current().stop();
//            })
//    );
//
//    system.run();

    const P: usize = 10; // Array size
    const N: usize = 2; // Number of actors
    const K: usize = 5; // Number of chunks

    let numbers = util::random_vec(P);
    let sort_req = msgs::SortingRequest::new(numbers, None);


    System::run(|| {
        let supervisor = Arbiter::start(|_| SupervisorActor::new("MASTER", N, K));
        let res = supervisor.send(sort_req);

        Arbiter::spawn(
            res
                .map(|r| {
                    println!("Result: Sorted(Vec[{}], Duration: {} (ms)]", r.values.len(), r.duration);
                    System::current().stop();
                })
                .map_err(|_| {
                    println!("Error occurred!");
                    System::current().stop();
                })
        );
    });
}
