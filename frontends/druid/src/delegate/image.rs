use crate::delegate::PartialDelegate;
use crate::state::State;
use crate::widgets::remote_image;
use druid::{Command, Env, ExtEventSink, ImageBuf, Target};
use futures::{future, future::BoxFuture, FutureExt};
use lru_cache::LruCache;

pub struct ImageDelegate {
    cache: LruCache<String, ImageBuf>,
}

const IMAGE_CACHE_SIZE: usize = 2048;

impl ImageDelegate {
    pub fn new() -> Self {
        ImageDelegate {
            cache: LruCache::new(IMAGE_CACHE_SIZE),
        }
    }
}

impl PartialDelegate for ImageDelegate {
    fn command(
        &mut self,
        sink: ExtEventSink,
        target: Target,
        cmd: &Command,
        _data: &mut State,
        _env: &Env,
    ) -> Option<BoxFuture<'static, ()>> {
        if let Some(location) = cmd.get(remote_image::REQUEST_DATA).cloned() {
            if let Some(image_buf) = self.cache.get_mut(&location).cloned() {
                let payload = remote_image::ImagePayload {
                    location,
                    image_buf,
                };
                sink.submit_command(remote_image::PROVIDE_DATA, payload, target)
                    .unwrap();
                Some(future::ready(()).boxed())
            } else {
                Some(
                    async move {
                        let req = surf::get(format!("http://localhost:8080{}", location));
                        let client = surf::Client::new().with(surf::middleware::Redirect::new(5));
                        let mut res = client.send(req).await.unwrap();

                        if res.status().is_success() {
                            let res = res.body_bytes().await.unwrap();
                            let image_buf = ImageBuf::from_data(&res).unwrap();
                            let payload = remote_image::ImagePayload {
                                location,
                                image_buf,
                            };
                            sink.submit_command(remote_image::PROVIDE_DATA, payload, target)
                                .unwrap();
                        }
                    }
                    .boxed(),
                )
            }
        } else if let Some(payload) = cmd.get(remote_image::PROVIDE_DATA).cloned() {
            self.cache.insert(payload.location, payload.image_buf);
            None
        } else {
            None
        }
    }
}
