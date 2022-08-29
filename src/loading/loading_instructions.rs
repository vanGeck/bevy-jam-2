use std::collections::HashMap;

use crate::game::FontId::*;
use crate::game::SoundId::*;
use crate::game::TextureId::*;
use crate::game::{ FontId, SoundId, TextureId};

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Default, Debug)]
pub struct LoadingConfig {
    pub textures: HashMap<TextureId, String>,
    pub atlases: HashMap<TextureId, String>,
    pub sfx: HashMap<SoundId, String>,
    pub fonts: HashMap<FontId, String>,
}

/// Prepares the LoadingConfig:
#[must_use]
pub fn prepare_loading_config() -> LoadingConfig {
    let mut c = LoadingConfig::default();

    c.textures.insert(NotFound, "not_found.png".to_string());
    c.textures
        .insert(UiPanelTexture, "MyPanel2.png".to_string());
    c.textures.insert(Overseer, "overseer.png".to_string());
    c.textures
        .insert(OverseerEyesWhite, "overseer_eyes_white.png".to_string());
    c.textures
        .insert(OverseerIris, "overseer_eyes_black.png".to_string());
    c.textures.insert(Vial, "Vial.png".to_string());
    c.textures
        .insert(TileEight, "Grid_Tile_8x8.png".to_string());
    c.textures
        .insert(TileSixteen, "Grid_Tile_16x16.png".to_string());
    c.textures
        .insert(TileThirtyTwo, "Grid_Tile_32x32.png".to_string());
    c.textures.insert(HerbRed, "HerbRed.png".to_string());
    c.textures.insert(HerbGreen, "HerbGreen.png".to_string());
    c.textures.insert(HerbViolet, "HerbViolet.png".to_string());
    c.textures
        .insert(EssenceAlacrity, "EssenceAlacrity.png".to_string());
    c.textures
        .insert(EssenceMight, "EssenceMight.png".to_string());
    c.textures
        .insert(EssenceVitality, "EssenceVitality.png".to_string());
    c.textures
        .insert(FlaskHealing, "FlaskHealing.png".to_string());
    c.textures
        .insert(FlaskStrength, "FlaskStrength.png".to_string());
    c.textures.insert(FlaskSkill, "FlaskSkill.png".to_string());
    c.textures
        .insert(FlaskToughness, "FlaskToughness.png".to_string());
    c.textures.insert(SwordRusty, "SwordRusty.png".to_string());
    c.textures.insert(Sword, "Sword.png".to_string());
    c.textures
        .insert(SwordMasterwork, "SwordMasterwork.png".to_string());
    c.textures
        .insert(SwordOfSpeed, "SwordSpeed.png".to_string());
    c.textures.insert(
        MasterworkSwordOfSpeed,
        "SwordMasterworkSpeed.png".to_string(),
    );
    c.textures
        .insert(SwordOfWounding, "SwordWounding.png".to_string());
    c.textures.insert(
        MasterworkSwordOfWounding,
        "SwordMasterworkWounding.png".to_string(),
    );
    c.textures
        .insert(CombineButton, "Combine_Button.png".to_string());
    c.textures.insert(Scroll, "Scroll.png".to_string());
    c.textures.insert(AxeRusty, "AxeRusty.png".to_string());
    c.textures.insert(Axe, "Axe.png".to_string());
    c.textures
        .insert(AxeMasterwork, "AxeMasterwork.png".to_string());
    c.textures.insert(ArmorRusty, "ArmorRusty.png".to_string());
    c.textures.insert(Armor, "Armor.png".to_string());
    c.textures
        .insert(ArmorMasterwork, "ArmorMasterwork.png".to_string());
    c.textures
        .insert(ShieldRusty, "ShieldRusty.png".to_string());
    c.textures.insert(Shield, "Shield.png".to_string());
    c.textures
        .insert(ShieldMasterwork, "ShieldMasterwork.png".to_string());

    c.sfx.insert(EnterRat, "monsters/rat/".to_string());
    c.sfx
        .insert(EnterLittleMonster, "monsters/little_monster/".to_string());
    c.sfx
        .insert(EnterBigMonster, "monsters/big_monster/".to_string());
    c.sfx
        .insert(EnterSkeleton, "monsters/skeleton/".to_string());
    c.sfx.insert(EnterZombie, "monsters/zombie/".to_string());
    c.sfx.insert(DoorCreak, "door_creak".to_string());
    c.sfx.insert(GoblinAhah, "goblin_ahah".to_string());
    c.sfx.insert(SlashHit, "slash_hit".to_string());
    c.sfx.insert(SwordClang, "sword_clang".to_string());
    c.sfx.insert(WaterDripping, "water_dripping".to_string());
    c.sfx.insert(Alchemy, "combining/alchemy.ogg".to_string());
    c.sfx
        .insert(CantCombine, "combining/cant_combine.ogg".to_string());
    c.sfx
        .insert(UpgradeWeapon, "combining/upgrade_weapon.ogg".to_string());

    c.fonts
        .insert(FiraSansLight, "FiraSans-Light.ttf".to_string());
    c.fonts
        .insert(FiraSansRegular, "FiraSans-Regular.ttf".to_string());
    c.fonts
        .insert(FiraSansMedium, "FiraSans-Medium.ttf".to_string());
    c.fonts
        .insert(FiraSansBold, "FiraSans-Bold.ttf".to_string());
    c
}
