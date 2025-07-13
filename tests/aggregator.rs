// tests/aggregator.rs

#[cfg(test)]
mod tests {
    use rigol_cli::aggregator::{Aggregator, CommLayer, InstrumentInfo};
    use super::*;
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::error::Error;

    struct DummyComm {
        ports: Vec<String>,
        devices: HashMap<String, Vec<(String, String)>>,
    }

    impl DummyComm {
        fn new() -> Self {
            let mut devs = HashMap::new();
            devs.insert(
                "p1".into(),
                vec![
                    ("A".into(), "DUMMY,DS1000Z,1234,1.0".into()),
                    ("B".into(), "DUMMY,DM3068,5678,1.0".into()),
                ],
            );
            Self {
                ports: vec!["p1".into()],
                devices: devs,
            }
        }
    }

    impl CommLayer for DummyComm {
        fn name(&self) -> &str {
            "DUMMY"
        }
        fn lsports(&self) -> Result<Vec<String>, Box<dyn Error>> {
            Ok(self.ports.clone())
        }
        fn configure_port(&mut self, _p: &str, _s: &Value) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn scan(
            &mut self,
            port: &str,
            _rng: Option<&Value>,
        ) -> Result<Vec<(String, String, Option<String>, Option<String>, Option<String>)>, Box<dyn Error>>
        {
            let mut out = Vec::new();
            if let Some(list) = self.devices.get(port) {
                for (id, idn) in list {
                    let p: Vec<&str> = idn.split(',').collect();
                    out.push((
                        port.into(),
                        id.into(),
                        p.get(0).map(|s| s.to_string()),
                        p.get(1).map(|s| s.to_string()),
                        None,
                    ));
                }
            }
            Ok(out)
        }
        fn send(
            &mut self,
            _port: &str,
            _id: &str,
            msg: &str,
        ) -> Result<Option<String>, Box<dyn Error>> {
            Ok(Some(format!("echo {}", msg.trim())))
        }
    }

    #[test]
    fn discover_and_echo() {
        let mut aggr = Aggregator {
            connected_instruments: HashMap::new(),
            next_uuid: 0,
            comm_layers: vec![Box::new(DummyComm::new())],
            config: Value::Null,
        };
        let db = aggr.discover_all();
        assert_eq!(db.len(), 2);
        let ids: Vec<u32> = db.keys().copied().collect();
        let r = aggr.send_to(&ids, "*IDN?");
        assert_eq!(r.len(), 2);
        for (_, res) in r {
            assert!(res.unwrap().starts_with("echo"));
        }
    }
}
