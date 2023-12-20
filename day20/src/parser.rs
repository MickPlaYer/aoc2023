use crate::structs::Module;
use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<String, Module> {
    let mut conjunction_names = Vec::new();
    let mut modules = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(" -> ");
        let module_tag = split.next().unwrap();
        let destinations = split
            .next()
            .unwrap()
            .split(", ")
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        if module_tag == "broadcaster" {
            let name = String::from("broadcaster");
            let module = Module::broadcaster(name.clone(), destinations);
            modules.insert(name, module);
        } else if module_tag.starts_with('%') {
            let name = String::from(&module_tag[1..]);
            let module = Module::flip_flop(name.clone(), destinations);
            modules.insert(name, module);
        } else if module_tag.starts_with('&') {
            let name = String::from(&module_tag[1..]);
            let module = Module::conjunction(name.clone(), destinations);
            conjunction_names.push(name.clone());
            modules.insert(name, module);
        } else {
            panic!("wrong module_tag: {:?}", module_tag);
        }
    }
    let mut memory_to_add = Vec::new();
    for (name, module) in &modules {
        for conjunction_name in &conjunction_names {
            if module.is_connect_to(&conjunction_name) {
                memory_to_add.push((conjunction_name.clone(), name.clone()));
            }
        }
    }
    for (conjunction_name, name) in memory_to_add {
        let conjunction = modules.get_mut(&conjunction_name).unwrap();
        conjunction.add_memory(name);
    }
    modules
}
