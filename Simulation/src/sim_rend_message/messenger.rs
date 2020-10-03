//use crate::sim_object::*;
use std::sync::mpsc::{channel, Receiver, Sender};
//use gdnative::{ToVariant, FromVariant};

use crate::common::*;
use crate::sim_rend_message::messages_rend::*;
use crate::sim_rend_message::messages_sim::*;

// this struct communicates between simulation and renderer
pub struct SimMessenger {
    //sim_send: Sender<Vec<EngineMessage>>,
    sim_send: Sender<Vec<SimMsg>>,
    sim_rec: Receiver<Vec<RenderMessage>>,
}

impl SimMessenger {
    // pub fn send(&self, msg: Vec<EngineMessage>) {
    pub fn send(&self, msg: Vec<SimMsg>) {
        self.sim_send.send(msg).unwrap();
    }

    pub fn rec(&self) -> Vec<RenderMessage> {
        let mut msg = self.sim_rec.try_iter();
        let mut ret: Vec<RenderMessage> = vec![];
        while let Some(mut x) = msg.next() {
            ret.append(&mut x);
        }
        ret
    }
}

pub struct RendMessenger {
    rend_send: Sender<Vec<RenderMessage>>,
    // rend_rec: Receiver<Vec<EngineMessage>>,
    rend_rec: Receiver<Vec<SimMsg>>,
}

impl RendMessenger {
    pub fn send(&self, msg: Vec<RenderMessage>) {
        self.rend_send.send(msg).unwrap();
    }

    // pub fn rec(&self) -> Vec<EngineMessage> {
    pub fn rec(&self) -> Vec<SimMsg> {
        let mut msg = self.rend_rec.try_iter();
        // let mut ret: Vec<EngineMessage> = vec![];
        let mut ret: Vec<SimMsg> = vec![];
        while let Some(mut x) = msg.next() {
            ret.append(&mut x);
        }
        ret
    }
}

pub fn create_messenger() -> (SimMessenger, RendMessenger) {
    // let (eng_send, rend_rec) = channel::<Vec<EngineMessage>>();
    let (eng_send, rend_rec) = channel::<Vec<SimMsg>>();
    let (rend_send, eng_rec) = channel::<Vec<RenderMessage>>();
    let ret1 = SimMessenger {
        sim_send: eng_send,
        sim_rec: eng_rec,
    };
    let ret2 = RendMessenger {
        rend_send: rend_send,
        rend_rec: rend_rec,
    };
    (ret1, ret2)
}
