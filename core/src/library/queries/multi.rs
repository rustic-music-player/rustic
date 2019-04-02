#[derive(Default, Clone, Copy)]
pub struct MultiQuery {
    pub joins: bool,
    pub limit: Option<usize>
}

impl MultiQuery {
    pub fn new() -> MultiQuery {
        MultiQuery::default()
    }

    pub fn joins(&mut self, joins: bool) -> &mut MultiQuery {
        self.joins = joins;
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut MultiQuery {
        self.limit = Some(limit);
        self
    }
}