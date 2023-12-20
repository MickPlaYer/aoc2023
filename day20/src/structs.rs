use std::collections::HashMap;

#[derive(Debug)]
pub enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

#[derive(Debug)]
pub struct Module {
    module_type: ModuleType,
    name: String,
    pub destinations: Vec<String>,
}
impl Module {
    pub(crate) fn broadcaster(name: String, destinations: Vec<String>) -> Self {
        Self {
            module_type: ModuleType::Broadcast,
            name,
            destinations,
        }
    }

    pub(crate) fn flip_flop(name: String, destinations: Vec<String>) -> Self {
        Self {
            module_type: ModuleType::FlipFlop(false),
            name,
            destinations,
        }
    }

    pub(crate) fn conjunction(name: String, destinations: Vec<String>) -> Module {
        Self {
            module_type: ModuleType::Conjunction(HashMap::new()),
            name,
            destinations,
        }
    }

    pub(crate) fn is_connect_to(&self, name: &String) -> bool {
        self.destinations.contains(name)
    }

    pub(crate) fn add_memory(&mut self, name: String) {
        if let ModuleType::Conjunction(memory) = &mut self.module_type {
            memory.insert(name, false);
        } else {
            panic!("add memory to wrong module: {:?}", self);
        }
    }

    pub(crate) fn send(&mut self, pulse: bool, from: String) -> Vec<(bool, String, String)> {
        let mut next_pulse = None;
        match &mut self.module_type {
            ModuleType::FlipFlop(value) => {
                if !pulse {
                    let new_value = !*value;
                    *value = new_value;
                    next_pulse = Some(new_value);
                }
            }
            ModuleType::Conjunction(memory) => {
                memory.insert(from, pulse);
                let all_high = memory.iter().all(|(_, v)| *v);
                next_pulse = Some(!all_high);
            }
            ModuleType::Broadcast => {
                next_pulse = Some(pulse);
            }
        }
        let mut pulses = Vec::new();
        if let Some(next_pluse) = next_pulse {
            for name in &self.destinations {
                pulses.push((next_pluse, name.clone(), self.name.clone()));
            }
        }
        pulses
    }
}
