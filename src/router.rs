use std::collections::{HashMap, HashSet};

pub trait Router: Send + Sync {
    fn route(&self, channel_id: usize) -> Option<&usize>;
    fn required_ports(&self) -> HashSet<usize>;
}

#[derive(Clone)]
pub struct StaticRouter {
    port_id: usize
}

impl StaticRouter {
    pub fn new(port_id: usize) -> Self {
        StaticRouter {
            port_id
        }
    }
}

impl Router for StaticRouter {
    fn route(&self, _: usize) -> Option<&usize> {
        return Some(&(self.port_id));
    }

    fn required_ports(&self) -> HashSet<usize> {
        let mut distinct_port_ids: HashSet<usize> = HashSet::new();
        distinct_port_ids.insert(self.port_id);
        return distinct_port_ids;
    }
}

#[derive(Clone)]
pub struct MapRouter {
    channel_id_to_port_id: HashMap<usize, usize>
}

impl MapRouter {
    pub fn new(channel_id_to_port_id: HashMap<usize, usize>) -> Self {
        return MapRouter {
            channel_id_to_port_id
        }
    }
}

impl Router for MapRouter {
    fn route(&self, channel_id: usize) -> Option<&usize>  {
        return self.channel_id_to_port_id.get(&channel_id);
    }

    fn required_ports(&self) -> HashSet<usize> {
        let mut distinct_port_ids: HashSet<usize> = HashSet::new();
        distinct_port_ids.extend(self.channel_id_to_port_id.values().into_iter());
        return distinct_port_ids;
    }
}

