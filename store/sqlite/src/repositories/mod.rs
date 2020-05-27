use failure::Error;

use rustic_core::{MultiQuery, SingleQuery};

pub use self::album::AlbumRepository;
pub use self::artist::ArtistRepository;
pub use self::track::TrackRepository;
pub use self::playlist::PlaylistRepository;

use rustic_core::library::Identifiable;

mod album;
mod artist;
mod track;
mod playlist;

pub trait Repository<TModel> where TModel: Identifiable {
    fn query(&self, query: SingleQuery) -> Result<Option<TModel>, Error>;
    fn query_all(&self, query: MultiQuery) -> Result<Vec<TModel>, Error>;

    fn insert(&self, model: &mut TModel) -> Result<(), Error>;
    fn insert_all(&self, models: &mut Vec<TModel>) -> Result<(), Error>;

    fn sync(&self, model: &mut TModel) -> Result<(), Error> {
        if let Some(_) = self.query(model.get_identifier().into())? {
            self.update(model)
        }else {
            self.insert(model)
        }
    }
    fn sync_all(&self, models: &mut Vec<TModel>) -> Result<(), Error> {
        for model in models.iter_mut() {
            self.sync(model)?;
        }
        Ok(())
    }

    fn update(&self, model: &mut TModel) -> Result<(), Error>;

    fn update_all(&self, models: &mut Vec<TModel>) -> Result<(), Error>;
}
