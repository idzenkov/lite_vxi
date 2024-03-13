use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::collections::{HashMap, HashSet};

/*TODO TAGS
ASSIGN
ACCEPT
DIALOGEPREPARE
DIALOGSTART
EXIT
REJECT
DISCONNECT
SEND
*/

fn test_logic(file_map: HashMap<Transition, Vec<Child>>, mut var_map: HashMap<String, String>) {
    for i in file_map.into_iter() {
        for j in i.1 {
            match j {
                Child::Assign(j) => {
                    tracing::info!("VAR_MAP BEFORE: {:#?}", var_map);
                    var_map.insert(j.name, j.value);
                    tracing::info!("VAR_MAP AFTER:{:#?}", var_map)
                }
                Child::Var(j) => {
                    var_map.insert(j.name, j.value);
                }
                Child::Log(j) => log(j),
            }
        }
    }
}

pub fn read_xml_test() -> (HashMap<Transition, Vec<Child>>, HashMap<String, String>) {
    let mut reader = Reader::from_file("./xml/start.xml").unwrap();
    let mut buf = Vec::new();
    let mut vec: Vec<Child> = Vec::new();
    let mut file_map: HashMap<Transition, Vec<Child>> = HashMap::new();
    let mut var_map: HashMap<String, String> = HashMap::new();
    let mut depth = 0;
    let mut transition = Transition::new();
    reader.trim_text(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"log" => {
                    if depth == 1 {
                        let mut log = Log::new();
                        log.expr = e
                            .attributes()
                            .map(|a| {
                                let attr = a.unwrap();
                                String::from_utf8(attr.value.to_vec()).unwrap()
                            })
                            .collect();
                        vec.push(Child::Log(log));
                        file_map.insert(transition.clone(), vec.clone());
                    }
                }
                b"var" => {
                    if depth == 1 {
                        let var = init(e.clone());
                        vec.push(Child::Var(var));
                        file_map.insert(transition.clone(), vec.clone());
                    }
                    if depth == 0 {
                        let var = init(e.clone());
                        var_map.insert(var.name, var.value);
                    }
                }
                b"assign" => {
                    let mut assign = Assign::new();
                    let name: String = e
                        .attributes()
                        .map(|a| {
                            let attr = a.unwrap();
                            let mut s = String::new();
                            if attr.key.0 == b"name" {
                                s = String::from_utf8(attr.value.to_vec()).unwrap();
                            }
                            s
                        })
                        .collect();

                    let value: String = e
                        .attributes()
                        .map(|a| {
                            let attr = a.unwrap();
                            let mut s = String::new();
                            if attr.key.0 == b"expr" {
                                s = String::from_utf8(attr.value.to_vec()).unwrap();
                            }
                            s
                        })
                        .collect();
                    assign.name = name;
                    assign.value = value;
                    vec.push(Child::Assign(assign));
                    file_map.insert(transition.clone(), vec.clone());
                }
                _ => (),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"transition" => {
                    depth += 1;
                    for i in e.attributes() {
                        let attr = i.unwrap();
                        if attr.key.0 == b"event" {
                            transition.event = String::from_utf8(attr.value.to_vec()).unwrap()
                        };
                        if attr.key.0 == b"state" {
                            transition.state = String::from_utf8(attr.value.to_vec()).unwrap()
                        }
                    }
                    file_map.insert(transition.clone(), Vec::new());
                    // tracing::info!("event: {0}, state: {1}", transition.event, transition.state);
                    // tracing::info!("{:?}", vec);
                    ()
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"transition" => {
                    depth -= 1;
                    vec = Vec::new();
                    transition = Transition::new();
                }
                _ => (),
            },
            _ => (),
        }
    }
    (file_map, var_map)
}

fn init(e: BytesStart<'_>) -> Var {
    let mut var = Var::new();
    let name: String = e
        .attributes()
        .map(|a| {
            let attr = a.unwrap();
            let mut s = String::new();
            if attr.key.0 == b"name" {
                s = String::from_utf8(attr.value.to_vec()).unwrap();
            }
            s
        })
        .collect();

    let value: String = e
        .attributes()
        .map(|a| {
            let attr = a.unwrap();
            let mut s = String::new();
            if attr.key.0 == b"expr" {
                s = String::from_utf8(attr.value.to_vec()).unwrap();
            }
            s
        })
        .collect();
    var.assign(name, value);
    var
}

pub fn log(log: Log) {
    tracing::info!("{:#?}", log);
}

fn assign(var_set: &HashSet<Var>, file_map: &HashMap<Transition, Vec<Child>>) {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
    pub event: String,
    pub state: String,
    //log: Vec<Log>,
    //var: Vec<Var>,
}

#[derive(Debug, Clone)]
pub struct Log {
    expr: String,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Child {
    Var(Var),
    Log(Log),
    Assign(Assign),
}

impl Var {
    fn new() -> Var {
        Var {
            name: String::new(),
            value: String::new(),
        }
    }
    fn assign(&mut self, name: String, value: String) {
        self.name = name;
        self.value = value;
    }
}

impl Assign {
    fn new() -> Assign {
        Assign {
            name: String::new(),
            value: String::new(),
        }
    }
}

impl Log {
    fn new() -> Log {
        Log {
            expr: String::new(),
            label: String::new(),
        }
    }
}

impl Transition {
    fn new() -> Transition {
        Transition {
            event: String::new(),
            state: String::new(),
            //log: Vec::new(),
            //var: Vec::new(),
        }
    }
}
