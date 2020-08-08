use crate::sim_fix_math::{FixF, Pos};
//use crate::sim_object::*;
use std::sync::mpsc::{channel, Receiver, Sender};
//use gdnative::{ToVariant, FromVariant};
use crate::sim_map::MapTile;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EngineMessage {
    //ObjSpawn(IdComp, Pos), // Deprecated
    ObjPosColl(u64, Pos, FixF), // Message carrying position and collision radius info
    ObjMove(u64, Pos),
    ObjNextPos(u64, Pos),
    ObjDest(u64, Pos),
    ObjPathTmp(u64, [Pos; 20]), // To visualise pathfinding. Sends next 10 steps.
    MapTile(Pos, MapTile),
    StructurePosTmp(u64, Pos),
    Fps(u64),
    None, // this message sucks
    //Break,
    //Object(ObjectID, Pos, Pos), // TODO remove this variant
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderMessage {
    Destination(u64, Pos),
    Spawn(Pos),
    SpawnStructureTmp(Pos),
    //None,
    Break,
}
// this struct communicates between simulation and renderer
pub struct SimMessenger {
    sim_send: Sender<Vec<EngineMessage>>,
    sim_rec: Receiver<Vec<RenderMessage>>,
}

impl SimMessenger {
    pub fn send(&self, msg: Vec<EngineMessage>) {
        self.sim_send.send(msg).unwrap();
    }

    pub fn rec(&self) -> Vec<RenderMessage> {
        let mut msg = self.sim_rec.try_iter();
        //let mut ret: Vec<RenderMessage> = vec![RenderMessage::None];
        let mut ret: Vec<RenderMessage> = vec![];
        while let Some(mut x) = msg.next() {
            ret.append(&mut x);
        }
        ret
    }
}

pub struct RendMessenger {
    rend_send: Sender<Vec<RenderMessage>>,
    rend_rec: Receiver<Vec<EngineMessage>>,
}

impl RendMessenger {
    pub fn send(&self, msg: Vec<RenderMessage>) {
        self.rend_send.send(msg).unwrap();
    }

    pub fn rec(&self) -> Vec<EngineMessage> {
        let mut msg = self.rend_rec.try_iter();
        //let mut ret: Vec<EngineMessage> = vec![EngineMessage::None];
        let mut ret: Vec<EngineMessage> = vec![];
        while let Some(mut x) = msg.next() {
            ret.append(&mut x);
        }
        ret
    }
}

pub fn create_messenger() -> (SimMessenger, RendMessenger) {
    let (eng_send, rend_rec) = channel::<Vec<EngineMessage>>();
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
