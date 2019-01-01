/*
Define the supervisor actor
*/
use actix::prelude::*;
use futures::Future;
use futures::future::{join_all};

use crate::msgs;
use crate::sorting_actor;


#[derive(Debug)]
pub struct SupervisorActor {
    pub id: String,
    num_actors: usize,
    num_chunks: usize,

    sorting_actors: Vec<Addr<sorting_actor::SortingActor>>,

    remaining_chunks: usize,
    sorted_values: Vec<i64>,
}

impl SupervisorActor {
    pub fn new(id: &str, num_actors: usize, num_chunks: usize) -> SupervisorActor {
        let id = String::from(id);
        let sorting_actors = vec![];
        let remaining_chunks = num_chunks;
        let sorted_values = vec![];

        SupervisorActor {
            id,
            num_actors,
            num_chunks,
            sorting_actors,
            remaining_chunks,
            sorted_values,
        }
    }

    pub fn spawn_sorting_actors(&mut self) {
        for i in 0..self.num_actors {
            self.sorting_actors.push(
                Arbiter::start(move |_| {
                    sorting_actor::SortingActor::new(&format!("SLAVE-{}", i))
                })
            );
        }
    }
}

impl Actor for SupervisorActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[{}] SupervisorActor ~ START", self.id);
        self.spawn_sorting_actors();
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("[{}] SupervisorActor ~ STOP", self.id);
    }
}

impl Handler<msgs::SortingRequest> for SupervisorActor {
    type Result = msgs::SortingResponse;


    fn handle(&mut self, msg: msgs::SortingRequest, ctx: &mut Context<Self>) -> Self::Result {
        let in_vec = msg.values;

        println!("[SupervisorActor] Got sorting request: Vec[{}]", in_vec.len());

        let chunks = split_vec(&in_vec, self.num_chunks);

        self.sorting_actors
            .iter()
            .cycle()
            .take(chunks.len())
            .zip(chunks)
            .for_each(|ac| {
                let (actor_addr, chunk) = ac;

                let msg = msgs::SortingRequest::new(chunk.to_vec(), Some(ctx.address().clone()));

                let _ = ctx.spawn(actor_addr.send(msg)
                    .map_err(|_| ())
                    .and_then(|_| {
                        println!("Supervisor: got response");
                        Ok(())
                    })
                    .into_actor(self)
                );
            });

//        let responses = assign_chunks(ctx.address(), &self.sorting_actors, &chunks);
//
//        for res in responses {
//            println!("Waiting for response!");
//            Arbiter::spawn(
//                res
//                    .map(|_| {
//                        println!("Finished sorting!");
//                    })
//                    .map_err(|_| {
//                        println!("Error occurred when waiting for sorting response!");
//                    })
//            );
//        }

//        let _ = ctx.spawn(self.sorting_actors[0].send(msgs::SortingRequest::new(vec![3,2,1], Some(ctx.address().clone())))
//            .map_err(|_| ())
//            .and_then(|_| {
//                println!("Supervisor: got response");
//                Ok(())
//            })
//            .into_actor(self)
//        );

        while self.remaining_chunks > 0 {
            std::thread::sleep_ms(1000);
            println!("There are still {} remaining chunks!", self.remaining_chunks);
        }

        msgs::SortingResponse::new(self.sorted_values.clone(), 0) // TODO: duration!
    }
}


impl Handler<msgs::SortingResponse> for SupervisorActor {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, msg: msgs::SortingResponse, _: &mut Context<Self>) -> Self::Result {
        println!("[SupervisorActor] Got SortingResponse!");

        if self.sorted_values.is_empty() {
            self.sorted_values = msg.values;
        }
        else {
            self.sorted_values = merge(&self.sorted_values, &msg.values);
        }

        self.remaining_chunks -= 1;

        Ok(())
    }
}


fn split_vec<T: Clone>(v: &Vec<T>, num_chunks: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let chunk_size = v.len() / num_chunks;

    if chunk_size == 0 {
        panic!("Chunk size is 0")
    }

    for c in v.chunks(chunk_size) {
        result.push(c.to_vec())
    }

    result
}


//fn assign_chunks(
//    supervisor: &mut SupervisorActor,
//    ctx: &mut SupervisorActor::Context,
////    supervisor_addr: Addr<SupervisorActor>,
//                 actors: &Vec<Addr<sorting_actor::SortingActor>>,
//                 chunks: &Vec<Vec<i64>>) -> Vec<Request<sorting_actor::SortingActor, msgs::SortingRequest>> {
//    actors
//        .iter()
//        .cycle()
//        .take(chunks.len())
//        .zip(chunks)
//        .map(|ac| {
//            let (actor_addr, chunk) = ac;
//
//            let msg = msgs::SortingRequest::new(chunk.to_vec(), Some(ctx.address().clone()));
//
//            let _ = ctx.spawn(actor_addr.send(msg)
//                .map_err(|_| ())
//                .and_then(|_| {
//                    println!("Supervisor: got response");
//                    Ok(())
//                })
//                .into_actor(supervisor)
//            );
////            actor_addr.send(msgs::SortingRequest::new(chunk.to_vec(), Some(supervisor_addr.clone())))
//        })
//        .collect()
//}


fn merge(v1: &Vec<i64>, v2: &Vec<i64>) -> Vec<i64> {
    let mut i = 0;
    let mut j = 0;

    let mut result: Vec<i64> = Vec::new();
    let total = v1.len() + v2.len();

    while result.len() != total {
        if i == v1.len() {
            result.extend_from_slice(&v2[j..]);
            break;
        }

        else if j == v2.len() {
            result.extend_from_slice(&v1[i..]);
            break;
        }

        else if v1[i] < v2[j] {
            result.push(v1[i]);
            i += 1;
        }

        else {
            result.push(v2[j]);
            j += 1;
        }
    }

    result
}
