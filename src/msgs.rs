/*
Define messages used in the system
*/
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;

use crate::supervisor;


#[derive(Debug)]
pub struct SortingRequest {
    pub values: Vec<i64>,
    pub from: Option<Addr<supervisor::SupervisorActor>>
}

impl Message for SortingRequest {
    type Result = SortingResponse;
}


impl SortingRequest {
    pub fn new(values: Vec<i64>, from: Option<Addr<supervisor::SupervisorActor>>) -> SortingRequest {
        SortingRequest{
            values,
            from
        }
    }
}


#[derive(Debug)]
pub struct SortingResponse {
    pub values: Vec<i64>,
    pub duration: i64
}

impl SortingResponse {
    pub fn new(values: Vec<i64>, duration: i64) -> SortingResponse {
        SortingResponse{
            values,
            duration
        }
    }
}

impl Message for SortingResponse {
    type Result = Result<(), std::io::Error>;
}

impl<A, M> MessageResponse<A, M> for SortingResponse
    where
        A: Actor,
        M: Message<Result = SortingResponse>
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
