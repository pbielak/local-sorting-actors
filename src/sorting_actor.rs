/*
Define the sorting actor
*/
use actix::prelude::*;
use futures::Future;

use crate::msgs;
use crate::util;


#[derive(Debug)]
pub struct SortingActor {
    pub id: String
}

impl SortingActor {
    pub fn new(id: &str) -> SortingActor {
        SortingActor {
            id: String::from(id)
        }
    }
}

impl Actor for SortingActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[{}] SortingActor ~ START", self.id)
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("[{}] SortingActor ~ STOP", self.id)
    }
}

impl Handler<msgs::SortingRequest> for SortingActor {
    type Result = msgs::SortingResponse;


    fn handle(&mut self, msg: msgs::SortingRequest, ctx: &mut Context<Self>) -> Self::Result {
        println!("[SortingActor][{}] Got sorting request: Vec[{}]", self.id, msg.values.len());

        let (vals, duration) = util::measure_time(&sort_vec, msg.values);

        let supervisor_addr = msg.from.expect("No supervisor address given");
        let _ = ctx.spawn(supervisor_addr.send(msgs::SortingResponse::new(vals, duration))
            .map_err(|_| ())
            .and_then(|_| {
                println!("Sorting actor: got response");
                Ok(())
            })
            .into_actor(self)
        );

//        Ok(())
        msgs::SortingResponse::new(vec![], -1)
    }
}

fn sort_vec<T: Clone + Ord>(v: Vec<T>) -> Vec<T> {
    let mut vals = v.clone();
    vals.sort();
    vals
}


