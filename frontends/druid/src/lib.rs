mod commands;
mod delegate;
mod icon;
mod state;
pub mod theme;
mod ui;
mod widgets;

use druid::AppLauncher;

use rustic_api::ApiClient;

use crate::delegate::RusticDelegate;
use crate::ui::main_window;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn start(client: ApiClient) -> Result<()> {
    let launcher = AppLauncher::with_window(main_window()).configure_env(self::theme::setup);
    let event_sink = launcher.get_external_handle();
    let delegate = RusticDelegate::new(event_sink, client)?;
    launcher.delegate(delegate).launch(Default::default())?;

    Ok::<(), failure::Error>(())
}
