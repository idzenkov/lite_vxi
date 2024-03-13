use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

pub fn parse_events() -> HashMap<String, String> {
    let mut reader = Reader::from_file("./xml/start.xml").unwrap();
    let mut buf = Vec::new();
    let mut event_map: HashMap<String, String> = HashMap::new();
    let mut external: String = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"event" => {
                    external = e
                        .attributes()
                        .map(|a| {
                            let attr = a.unwrap();
                            let mut s = String::new();
                            if attr.key.0 == b"external" {
                                s = String::from_utf8(attr.value.to_vec()).unwrap();
                            }
                            s
                        })
                        .collect();
                    event_map.insert(external.clone(), String::new());
                }
                _ => (),
            },
            Ok(Event::Text(e)) => {
                let ccxml = String::from_utf8(e.into_inner().to_vec()).unwrap();
                event_map.insert(external.clone(), ccxml);
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"event" => external = String::new(),
                _ => (),
            },
            _ => (),
        }
    }
}
