use std::collections::HashMap;

use crate::game::AlbumId::*;
use crate::game::FontId::*;
use crate::game::SoundId::*;
use crate::game::TextureId::*;
use crate::game::{AlbumId, FontId, SoundId, TextureId};

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Default, Debug)]
pub struct LoadingConfig {
    pub textures: HashMap<TextureId, String>,
    pub atlases: HashMap<TextureId, String>,
    pub sfx: HashMap<SoundId, String>,
    pub music: HashMap<AlbumId, String>,
    pub fonts: HashMap<FontId, String>,
}

impl LoadingConfig {
    /// Prepares the LoadingConfig:
    #[must_use]
    pub fn prepare() -> LoadingConfig {
        let mut config = LoadingConfig::default();

        config
            .textures
            .insert(NotFound, "not_found.png".to_string());
        config
            .textures
            .insert(UiPanelTexture, "MyPanel2.png".to_string());
        config.textures.insert(Overseer, "overseer.png".to_string());
        config
            .textures
            .insert(OverseerEyesWhite, "overseer_eyes_white.png".to_string());
        config
            .textures
            .insert(OverseerIris, "overseer_eyes_black.png".to_string());
        config.textures.insert(Vial, "Vial.png".to_string());
        config
            .textures
            .insert(TileEight, "Grid_Tile_8x8.png".to_string());
        config
            .textures
            .insert(TileSixteen, "Grid_Tile_16x16.png".to_string());
        config
            .textures
            .insert(TileThirtyTwo, "Grid_Tile_32x32.png".to_string());
        config.textures.insert(HerbRed, "HerbRed.png".to_string());
        config
            .textures
            .insert(HerbGreen, "HerbGreen.png".to_string());
        config
            .textures
            .insert(HerbViolet, "HerbViolet.png".to_string());
        config
            .textures
            .insert(EssenceAlacrity, "EssenceAlacrity.png".to_string());
        config
            .textures
            .insert(EssenceMight, "EssenceMight.png".to_string());
        config
            .textures
            .insert(EssenceVitality, "EssenceVitality.png".to_string());
        config
            .textures
            .insert(FlaskHealing, "FlaskHealing.png".to_string());
        config
            .textures
            .insert(FlaskStrength, "FlaskStrength.png".to_string());
        config
            .textures
            .insert(FlaskSkill, "FlaskSkill.png".to_string());
        config
            .textures
            .insert(FlaskToughness, "FlaskToughness.png".to_string());
        config
            .textures
            .insert(SwordRusty, "SwordRusty.png".to_string());
        config.textures.insert(Sword, "Sword.png".to_string());
        config
            .textures
            .insert(SwordMasterwork, "SwordMasterwork.png".to_string());
        config
            .textures
            .insert(SwordOfSpeed, "SwordSpeed.png".to_string());
        config.textures.insert(
            MasterworkSwordOfSpeed,
            "SwordMasterworkSpeed.png".to_string(),
        );
        config
            .textures
            .insert(SwordOfWounding, "SwordWounding.png".to_string());
        config.textures.insert(
            MasterworkSwordOfWounding,
            "SwordMasterworkWounding.png".to_string(),
        );
        config
            .textures
            .insert(CombineButton, "Combine_Button.png".to_string());
        config.textures.insert(Scroll, "Scroll.png".to_string());
        config.textures.insert(AxeRusty, "AxeRusty.png".to_string());
        config.textures.insert(Axe, "Axe.png".to_string());
        config
            .textures
            .insert(AxeMasterwork, "AxeMasterwork.png".to_string());
        config
            .textures
            .insert(ArmorRusty, "ArmorRusty.png".to_string());
        config.textures.insert(Armor, "Armor.png".to_string());
        config
            .textures
            .insert(ArmorMasterwork, "ArmorMasterwork.png".to_string());
        config
            .textures
            .insert(ShieldRusty, "ShieldRusty.png".to_string());
        config.textures.insert(Shield, "Shield.png".to_string());
        config
            .textures
            .insert(ShieldMasterwork, "ShieldMasterwork.png".to_string());

        config.atlases.insert(NotFound, "not_found.ron".to_string());
        config.atlases.insert(Cursor, "cursor.ron".to_string());
        config
            .atlases
            .insert(RecordPlayer, "record_player.ron".to_string());
        config.atlases.insert(Backpack, "backpack.ron".to_string());

        config.sfx.insert(EnterRat, "monsters/rat/".to_string());
        config
            .sfx
            .insert(EnterLittleMonster, "monsters/little_monster/".to_string());
        config
            .sfx
            .insert(EnterBigMonster, "monsters/big_monster/".to_string());
        config
            .sfx
            .insert(EnterSkeleton, "monsters/skeleton/".to_string());
        config
            .sfx
            .insert(EnterZombie, "monsters/zombie/".to_string());
        config.sfx.insert(DoorCreak, "door_creak".to_string());
        config.sfx.insert(GoblinAhah, "goblin_ahah".to_string());
        config.sfx.insert(SlashHit, "slash_hit".to_string());
        config.sfx.insert(SwordClang, "sword_clang".to_string());
        config
            .sfx
            .insert(WaterDripping, "water_dripping".to_string());
        config
            .sfx
            .insert(Alchemy, "combining/alchemy.ogg".to_string());
        config
            .sfx
            .insert(CantCombine, "combining/cant_combine.ogg".to_string());
        config
            .sfx
            .insert(UpgradeWeapon, "combining/upgrade_weapon.ogg".to_string());

        config.music.insert(Jazz, "jazz/".to_string());
        config.music.insert(Ominous, "ominous/".to_string());

        config
            .fonts
            .insert(FiraSansLight, "FiraSans-Light.ttf".to_string());
        config
            .fonts
            .insert(FiraSansRegular, "FiraSans-Regular.ttf".to_string());
        config
            .fonts
            .insert(FiraSansMedium, "FiraSans-Medium.ttf".to_string());
        config
            .fonts
            .insert(FiraSansBold, "FiraSans-Bold.ttf".to_string());
        config
    }
}
