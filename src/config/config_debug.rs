use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "db168435-8fa5-40f8-908f-560f30e6b158"]
pub struct DebugConfig {
    /// Filters logs using the [`EnvFilter`] format
    pub log_filter: String,
    pub show_debug_window: bool,
    /// If true, the loader will bypass the menu and drop you straight into the game.
    /// Can be very handy for rapid testing, not having to click the play button every time.
    pub skip_straight_to_game: bool,
    pub launch_fullscreen: bool,
}

#[derive(Default)]
pub struct DebugConfigLoader;

impl AssetLoader for DebugConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<DebugConfig>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["debug.ron"]
    }
}
