use std::path::PathBuf;

use std::str::FromStr;
use structopt::StructOpt;

use crate::config::Config;

#[derive(StructOpt, Debug)]
pub(crate) struct CliOptions {
    /// Verbosity (-v = debug, -vv = trace)
    #[structopt(short, long, parse(from_occurrences))]
    pub(crate) verbose: u8,

    /// Config file
    #[structopt(short, long, default_value = "config.toml", parse(from_os_str))]
    pub(crate) config: PathBuf,

    /// Extensions path
    #[structopt(long = "extensions")]
    pub(crate) extensions_path: Option<String>,

    /// Connect to remote instance
    #[structopt(long)]
    #[cfg(any(feature = "http-client"))]
    pub(crate) connect: Option<String>,

    /// Run headless
    #[structopt(long, short = "H")]
    pub(crate) headless: bool,

    #[structopt(long = "disable", short)]
    pub(crate) disabled_modules: Vec<Module>,
}

#[derive(Debug, Copy, Clone)]
pub enum Module {
    #[cfg(feature = "iced-frontend")]
    IcedFrontend,
    #[cfg(feature = "druid-frontend")]
    DruidFrontend,
    #[cfg(feature = "http-frontend")]
    HttpFrontend,
    #[cfg(feature = "mpd-frontend")]
    MpdFrontend,
    #[cfg(feature = "pocketcasts-provider")]
    PocketcastsProvider,
    #[cfg(feature = "soundcloud-provider")]
    SoundcloudProvider,
    #[cfg(feature = "spotify-provider")]
    SpotifyProvider,
    #[cfg(feature = "local-provider")]
    LocalProvider,
    #[cfg(feature = "youtube-provider")]
    YoutubeProvider,
    #[cfg(feature = "ytmusic-provider")]
    YouTubeMusicProvider,
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Module::*;

        match s {
            #[cfg(feature = "iced-frontend")]
            "iced-frontend" => Ok(IcedFrontend),
            #[cfg(feature = "druid-frontend")]
            "druid-frontend" => Ok(DruidFrontend),
            #[cfg(feature = "http-frontend")]
            "http-frontend" => Ok(HttpFrontend),
            #[cfg(feature = "mpd-frontend")]
            "mpd-frontend" => Ok(MpdFrontend),
            #[cfg(feature = "pocketcasts-provider")]
            "pocketcasts" => Ok(PocketcastsProvider),
            #[cfg(feature = "soundcloud-provider")]
            "soundcloud" => Ok(SoundcloudProvider),
            #[cfg(feature = "spotify-provider")]
            "spotify" => Ok(SpotifyProvider),
            #[cfg(feature = "local-provider")]
            "local" => Ok(LocalProvider),
            #[cfg(feature = "youtube-provider")]
            "youtube" => Ok(YoutubeProvider),
            #[cfg(feature = "ytmusic-provider")]
            "ytmusic" => Ok(YouTubeMusicProvider),
            _ => Err(format!("unknown module {}", s)),
        }
    }
}

impl Module {
    pub(crate) fn remove_disabled_module_config(&self, config: &mut Config) {
        use Module::*;

        match self {
            #[cfg(feature = "iced-frontend")]
            IcedFrontend => config.frontend.iced = None,
            #[cfg(feature = "druid-frontend")]
            DruidFrontend => config.frontend.druid = None,
            #[cfg(feature = "http-frontend")]
            HttpFrontend => config.frontend.http = None,
            #[cfg(feature = "mpd-frontend")]
            MpdFrontend => config.frontend.mpd = None,
            #[cfg(feature = "pocketcasts-provider")]
            PocketcastsProvider => config.provider.pocketcasts = None,
            #[cfg(feature = "soundcloud-provider")]
            SoundcloudProvider => config.provider.soundcloud = None,
            #[cfg(feature = "spotify-provider")]
            SpotifyProvider => config.provider.spotify = None,
            #[cfg(feature = "local-provider")]
            LocalProvider => config.provider.local = None,
            #[cfg(feature = "youtube-provider")]
            YoutubeProvider => config.provider.youtube = None,
            #[cfg(feature = "ytmusic-provider")]
            YouTubeMusicProvider => config.provider.ytmusic = None,
        }
    }
}
