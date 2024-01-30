use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::instruction::InstructionKind;

#[derive(Copy, Clone, Debug)]
struct InstructionHashWrapper {
    pub kind: InstructionKind,
}

impl PartialEq for InstructionHashWrapper {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(&self.kind) == std::mem::discriminant(&other.kind)
    }
}

impl Eq for InstructionHashWrapper {}

impl Hash for InstructionHashWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(&self.kind).hash(state);
    }
}

pub struct InstructionHandler {
    time_target_to_inst: HashMap<(u128, u128), HashSet<InstructionHashWrapper>>,
    target_to_times: HashMap<u128, HashMap<u128, u128>>,
    time_to_targets: HashMap<u128, HashMap<u128, u128>>
}


impl InstructionHandler {

    pub fn new() -> Self {
        Self {
            time_target_to_inst: HashMap::new(),
            target_to_times: HashMap::new(),
            time_to_targets: HashMap::new()
        }
    }
    pub fn insert(&mut self, target: u128, time: u128, kind: InstructionKind) {
        let wrapper = InstructionHashWrapper { kind };
        self.time_target_to_inst.entry((target, time)).and_modify(|set| {
            if set.contains(&wrapper) { set.remove(&wrapper); }
            set.insert(wrapper);
        }).or_insert(HashSet::from([wrapper]));
        self.target_to_times.entry(target).and_modify(|map| {
            map.entry(time).and_modify(|u|{ *u += 1 });
        }).or_insert(HashMap::from([(time, 1)]));
        self.target_to_times.entry(time).and_modify(|map| {
            map.entry(target).and_modify(|u|{ *u += 1 });
        }).or_insert(HashMap::from([(target, 1)]));
    }

    pub fn get(&self, target: u128, time: u128) -> Vec<InstructionKind> {
        self.time_target_to_inst.get(&(target, time))
            .unwrap_or(&HashSet::new())
            .iter()
            .fold(vec![], |mut acc,x| {acc.push(x.kind); acc})
    }

    pub fn has_type(&self, target: u128, time: u128, kind: InstructionKind) -> bool {
        match self.time_target_to_inst.get(&(target, time)) {
            Some(x) => x.contains(&InstructionHashWrapper { kind }),
            None => false
        }
    }

    pub fn has(&self, target: u128, time: u128, kind: InstructionKind) -> bool {
        match self.time_target_to_inst.get(&(target, time)) {
            Some(set) => match set.get(&InstructionHashWrapper {kind} ) {
                Some(inst) => {
                    inst.kind == kind
                }
                None => false
            },
            None => false
        }
    }

    pub fn remove_type(&mut self, target: u128, time: u128, kind: InstructionKind) {
        let wrapper = InstructionHashWrapper { kind };
        self.time_target_to_inst.entry((target, time)).and_modify(|set|{
            set.remove(&wrapper);
        });
        self.target_to_times.entry(target).and_modify(|map|{
            let mut del = false;
            map.entry(time).and_modify(|n|{
                *n -= 1;
                if *n == 0 {
                    del = true;
                }
            });
            map.remove(&time);
        });
        self.time_to_targets.entry(time).and_modify(|map|{
            let mut del = false;
            map.entry(target).and_modify(|n|{
                *n -= 1;
                if *n == 0 {
                    del = true;
                }
            });
            map.remove(&target);
        });

    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::InstructionKind;
    use crate::instruction_handler::{InstructionHandler, InstructionHashWrapper};
    use crate::instrument::oscillator::Waveform;

    #[test]
    fn instruction_equalities() {
        let x = InstructionHashWrapper { kind: InstructionKind::Waveform(Waveform::Square) };
        let y = InstructionHashWrapper { kind: InstructionKind::Waveform(Waveform::Sine) };
        let z = InstructionHashWrapper { kind: InstructionKind::Frequency(3.0) };
        let w = InstructionHashWrapper { kind: InstructionKind::Frequency(4.0) };
        assert_eq!(x,y);
        assert_eq!(z,w);
        assert_ne!(x,z);
    }

    #[test]
    fn handler_works() {
        let mut handler = InstructionHandler::new();
        handler.insert(0,0, InstructionKind::Waveform(Waveform::Square));
        handler.insert(0,1, InstructionKind::Frequency(3.0));
        assert_eq!(*handler.get(0,1).get(0).unwrap(), InstructionKind::Frequency(3.0));
        assert_eq!(handler.get(0,1).len(), 1);

        handler.insert(0,1, InstructionKind::Frequency(4.0));
        assert_eq!(handler.get(0,1).len(), 1);
        assert_eq!(*handler.get(0,1).get(0).unwrap(), InstructionKind::Frequency(4.0));

        handler.insert(0,1, InstructionKind::Note(4));
        assert_eq!(handler.get(0,1).len(), 2);

        assert!(handler.has(0,1, InstructionKind::Note(4)));
        assert!(!handler.has(0,1, InstructionKind::Note(3)));
        assert!(handler.has_type(0,1, InstructionKind::Note(3)));
    }
}