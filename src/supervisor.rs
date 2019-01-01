/*
Define the supervisor actor
*/
use actix::prelude::*;
use futures::Future;

use crate::msgs;
use crate::sorting_actor;


#[derive(Debug)]
pub struct SupervisorActor {
    pub id: String,
    num_actors: usize,
    num_chunks: usize,

    sorting_actors: Vec<Addr<sorting_actor::SortingActor>>,
}

impl SupervisorActor {
    pub fn new(id: &str, num_actors: usize, num_chunks: usize) -> SupervisorActor {
        let id = String::from(id);
        let sorting_actors = vec![];

        SupervisorActor {
            id,
            num_actors,
            num_chunks,
            sorting_actors,
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


    fn handle(&mut self, msg: msgs::SortingRequest, _: &mut Context<Self>) -> Self::Result {
        let in_vec = msg.values;

        println!("[SupervisorActor] Got sorting request: Vec[{}]", in_vec.len());

        let chunks = split_vec(&in_vec, self.num_chunks);

        let tasks: Vec<Request<sorting_actor::SortingActor, msgs::SortingRequest>> = self.sorting_actors
            .iter()
            .cycle()
            .take(chunks.len())
            .zip(chunks)
            .map(|ac| {
                let (actor, chunk) = ac;
                actor.send(msgs::SortingRequest::new(chunk.to_vec()))
            })
            .collect();

        let x = tasks.into_iter().fold(msgs::SortingResponse::new(vec![], 0), |acc, task| {
            let res: msgs::SortingResponse = task.wait().unwrap();

            msgs::SortingResponse::new(merge(&acc.values, &res.values), acc.duration + res.duration)
        });

        x
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
