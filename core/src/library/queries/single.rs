#[derive(Default, Clone)]
pub struct SingleQuery {
    pub identifier: SingleQueryIdentifier,
    pub joins: bool,
}

#[derive(Clone)]
pub enum SingleQueryIdentifier {
    Id(usize),
    Uri(String)
}

impl Default for SingleQueryIdentifier {
    fn default() -> SingleQueryIdentifier {
        SingleQueryIdentifier::Id(0)
    }
}

impl SingleQuery {
    pub fn id(id: usize) -> SingleQuery {
        SingleQuery {
            identifier: SingleQueryIdentifier::Id(id),
            ..SingleQuery::default()
        }
    }

    pub fn uri(uri: String) -> SingleQuery {
        SingleQuery {
            identifier: SingleQueryIdentifier::Uri(uri),
            ..SingleQuery::default()
        }
    }

    pub fn joins(&mut self, joins: bool) -> &mut SingleQuery {
        self.joins = joins;
        self
    }
}