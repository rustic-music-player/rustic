use super::{NavigationError, ProviderFolder, SharedProviders};
use failure::Error;

pub struct Explorer {
    pub path: Vec<String>,
    providers: SharedProviders,
}

impl Explorer {
    pub fn new(providers: SharedProviders) -> Explorer {
        Explorer {
            path: vec![],
            providers,
        }
    }

    pub fn navigate_absolute<'a>(&mut self, path: &'a str) {
        let mut absolute = vec![];
        let mut current = path;
        while !current.is_empty() {
            let layer = match current.find('/') {
                Some(index) => {
                    let layer = &current[..index];
                    current = &current[index + 1..];
                    layer
                }
                None => {
                    let copy = current;
                    current = "";
                    copy
                }
            };
            absolute.push(layer.to_owned());
        }
        self.path = absolute;
    }

    pub fn navigate(&mut self, path: String) {
        self.path.push(path);
    }

    pub fn go_up(&mut self) {
        self.path.pop();
    }

    pub fn path(&self) -> String {
        self.path.iter().fold(String::new(), |mut a, b| {
            a.push_str(format!("{}/", b).as_str());
            a
        })
    }

    fn get_root(&self) -> ProviderFolder {
        let folders = self
            .providers
            .iter()
            .map(|provider| provider.title())
            .collect();
        ProviderFolder {
            folders,
            items: vec![],
        }
    }

    pub async fn items(&self) -> Result<ProviderFolder, Error> {
        let root = self.get_root();
        match self.path.len() {
            0 => Ok(root),
            1 => {
                let path = &self.path[0];
                let provider = self
                    .providers
                    .iter()
                    .find(|provider| &provider.title() == path);
                match provider {
                    Some(provider) => Ok(provider.get().await.root()),
                    None => Err(Error::from(NavigationError::PathNotFound))
                }
            }
            _ => {
                let path = &self.path[0];
                let provider = self
                    .providers
                    .iter()
                    .find(|provider| &provider.title() == path);
                let path = &self.path[1..];
                match provider {
                    Some(provider) => provider.get().await.navigate(path.to_vec()).await,
                    None => Err(Error::from(NavigationError::PathNotFound))
                }
            }
        }
    }
}
