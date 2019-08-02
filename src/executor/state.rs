use std::collections::HashMap;
use std::collections::HashSet;
use super::protobuf::state::StateBytes;
use quick_protobuf::deserialize_from_slice;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Query {
    outer: [u8;32],
    edge: [u8;32],
}

pub struct State {
    pub object_id: [u8;32],
    pub state: [u8;32],
    matrix: HashMap<Query,[u8;32]>,
}

impl State {
    pub fn new(_s: &Vec<u8>) -> Self{
        let pre_state: StateBytes = deserialize_from_slice(_s).expect("parse error");
        let mut object_id = [0u8;32];
        object_id.copy_from_slice(pre_state.object_id.to_vec().as_slice());

        let mut state = [0u8;32];
        state.copy_from_slice(pre_state.state.to_vec().as_slice());

        let mut matrix = HashMap::<Query,[u8;32]>::new();
        for (_key, _value) in pre_state.matrix.iter() {
            let key = _key.to_vec();
            let (_outer, _edge) = key.split_at(32);

            let mut outer = [0u8;32];
            outer.copy_from_slice(_outer);

            let mut edge = [0u8;32];
            edge.copy_from_slice(_edge);

            let _query = Query {
                outer, edge
            };

            let mut value = [0u8;32];
            value.copy_from_slice(_value);

            matrix.insert(_query, value);
        }

        State {
            object_id,
            state,
            matrix,
        }
    }

    pub fn transform(&self, _op: &[u8;32]) {

    }

    pub fn flush(self){

    }
}

