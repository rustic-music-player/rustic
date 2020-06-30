pub trait Aggregate<M>: From<M> {
    fn add_entry(&mut self, model: M);
    fn contains(&self, model: &M) -> bool;

    fn aggregate(models: Vec<M>) -> Vec<Self> {
        models
            .into_iter()
            .fold(Vec::<Self>::new(), |mut aggregated, model| {
                if let Some(aggregation) = aggregated.iter_mut().find(|m| m.contains(&model)) {
                    aggregation.add_entry(model);
                } else {
                    let aggregation = Self::from(model);
                    aggregated.push(aggregation);
                }
                aggregated
            })
    }
}
