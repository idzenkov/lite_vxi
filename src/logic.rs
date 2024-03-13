use std::collections::HashMap;

use crate::parser::{Transition,Child, log};

pub fn find_event_name(buf:Vec<u8>){
    let buf_str = String::from_utf8(buf).unwrap();
    buf_str.contains("Event-Name: ");
}

pub fn matching_fsevent(event_name:String, event_map: HashMap<String, String>) -> String {
    event_map.get(&event_name).unwrap().to_owned()
}

pub fn get_event_value(event_name:String, event_map: HashMap<Transition, Vec<Child>>) -> Vec<Child> {
    let key:Transition = Transition{ event : event_name, state : String::from("") };
    event_map.get(&key).unwrap().to_owned()
}

pub fn logic(vec:Vec<Child>, mut var_map: HashMap<String,String>) {
    for i in vec {
        match i {
            Child::Var(i) => {var_map.insert(i.name,i.value );},
            Child::Assign(a) => {var_map.insert(a.name,a.value );},
            Child::Log(l) => log(l),
        }
    }
}
