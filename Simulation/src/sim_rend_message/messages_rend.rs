use crate::common::*;
use crate::sim_fix_math::*;

// #[derive(Debug, PartialEq, Clone, Copy)]
#[derive(Debug, Clone, Copy)]
pub enum RenderMessage {
    Destination(UId, PId, Pos),
    Spawn(PId, Pos),
    SpawnSmart(PId, Pos),
    //SpawnStructureTmp(Pos, PId),
    UseAbility(UId, PId, AbilityID, ObjTarget),
    InputOrder(PId, [Option<UId>; UNIT_GROUP_CAP], UnitOrder), //
    Break,
}

/// Horribly written function that converts vector to vector of Arrays  
pub fn make_unit_group(units: &Vec<UId>) -> Vec<[Option<UId>; UNIT_GROUP_CAP]> {
    let mut groups_num = units.len() / UNIT_GROUP_CAP;

    if (units.len() % UNIT_GROUP_CAP) > 0 {
        groups_num += 1;
    }

    let mut ret: Vec<[Option<UId>; UNIT_GROUP_CAP]> = vec![];

    for i in 0..groups_num {
        let group_id_start = i * UNIT_GROUP_CAP;
        let current_group_size = (units.len() - group_id_start).min(UNIT_GROUP_CAP);

        let mut base_array = [None; UNIT_GROUP_CAP];

        for unit_i in group_id_start..(group_id_start + current_group_size) {
            base_array[unit_i - group_id_start] = Some(units[unit_i]);
        }

        ret.push(base_array)
    }

    ret
}

#[cfg(test)]
mod rend_msg_test {

    use crate::common::UId;
    use crate::sim_rend_message::messages_rend::make_unit_group;

    #[test]
    /// This test will break if common::types::UNIT_GROUP_CAP =/= 32
    fn make_unit_group_test() {
        let units = vec![1; 33];
        let group = make_unit_group(&units);
        assert_eq!(group.len(), 2);

        let units = vec![1; 5];
        let group = make_unit_group(&units);
        assert_eq!(group.len(), 1);

        let units = vec![1; 64];
        let group = make_unit_group(&units);
        assert_eq!(group.len(), 2);

        let units = vec![1; 65];
        let group = make_unit_group(&units);
        assert_eq!(group.len(), 3);

        let units: Vec<UId> = vec![];
        let group = make_unit_group(&units);
        assert_eq!(group.len(), 0);

        let mut units = vec![1; 65];
        units[32] = 2;
        let group = make_unit_group(&units);
        assert_eq!(group[1][0], Some(2));
    }
}
