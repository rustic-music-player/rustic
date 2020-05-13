use iced::futures;
use log::trace;
use rustic_api::models::SyncStateModel;
use rustic_api::ApiClient;

pub struct SyncRecipe {
    api: ApiClient,
}

impl SyncRecipe {
    pub fn new(api: ApiClient) -> Self {
        SyncRecipe { api }
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for SyncRecipe
where
    H: std::hash::Hasher,
{
    type Output = SyncStateModel;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        trace!("stream");
        let api = self.api.clone();
        api.sync_state()
    }
}
