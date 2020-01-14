use std::sync::Arc;

use failure::err_msg;

use rustic_core::{Provider, Rustic};
use viewmodels::*;

pub fn get_providers(rustic: &Arc<Rustic>) -> Vec<ProviderModel> {
    rustic
        .providers
        .iter()
        .map(|provider| {
            let provider = provider.read().unwrap();
            let title = provider.title().to_owned();
            let provider_type = provider.provider();
            let explore = provider.root();

            ProviderModel {
                title,
                provider: provider_type,
                explore,
            }
        })
        .collect()
}

pub fn navigate(
    rustic: &Arc<Rustic>,
    provider_type: Provider,
    path: &str,
) -> Result<ProviderFolderModel, failure::Error> {
    let provider = rustic
        .providers
        .iter()
        .find(|provider| provider.read().unwrap().provider() == provider_type)
        .ok_or_else(|| err_msg("Invalid provider"))?;

    let provider = provider.read().unwrap();
    let path = path.split("/").map(String::from).collect();
    let folder = provider.navigate(path)?;
    let folder = ProviderFolderModel::new(folder);

    Ok(folder)
}
