use bevy::prelude::*;
use bevy_asset_loader::{
    AssetCollection, AssetCollectionApp,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
        // .init_collection::<AudioAssets>();
    }
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "uncolored_desert.png")]
    pub background: Handle<Image>,
    #[asset(path = "glassPanel.png")]
    pub panel: Handle<Image>,
    #[asset(path = "blue_button11.png")]
    pub button: Handle<Image>,
    #[asset(path = "blue_button12.png")]
    pub button_pressed: Handle<Image>,
    #[asset(path = "grey_box.png")]
    pub box_unchecked: Handle<Image>,
    #[asset(path = "green_boxCheckmark.png")]
    pub box_checked: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 3,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "grass.png")]
    pub grass: Handle<TextureAtlas>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "gameover.ogg")]
//     pub gameover:
// Handle<bevy_kira_audio::AudioSource>,
//     #[asset(path = "apple.ogg")]
//     pub apple:
// Handle<bevy_kira_audio::AudioSource>, }
