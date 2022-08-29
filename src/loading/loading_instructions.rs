use std::collections::HashMap;

use crate::game::FontId::*;
use crate::game::SoundId::*;
use crate::game::TextureId::*;
use crate::game::{FontId, SoundId, TextureId};

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Default, Debug)]
pub struct LoadingConfig {
    pub textures: HashMap<TextureId, String>,
    pub atlases: HashMap<TextureId, String>,
    pub sfx: HashMap<SoundId, Vec<String>>,
    pub fonts: HashMap<FontId, String>,
}

/// Prepares the LoadingConfig:
#[must_use]
pub fn prepare_loading_config() -> LoadingConfig {
    let mut c = LoadingConfig::default();

    // ==============================================================
    // ================= Textures
    // ==============================================================

    c.textures.insert(NotFound, "not_found.png".to_string());
    c.textures
        .insert(UiPanelTexture, "MyPanel2.png".to_string());
    c.textures
        .insert(MenuCaveBg, "menu_cave_bg.png".to_string());
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


    // ==============================================================
    // ================= SFX
    // ==============================================================

    let mut vec = Vec::new();
    vec.push("combine_alchemy/alchemy.ogg".to_string());
    c.sfx.insert(CombineAlchemy, vec);

    let mut vec = Vec::new();
    vec.push("combine_cant/combine_cant.ogg".to_string());
    c.sfx.insert(CombineCant, vec);

    let mut vec = Vec::new();
    vec.push("combine_smithing/upgrade_weapon.ogg".to_string());
    c.sfx.insert(CombineSmithing, vec);

    let mut vec = Vec::new();
    vec.push("door_creak/door1.ogg".to_string());
    vec.push("door_creak/door2.ogg".to_string());
    c.sfx.insert(DoorCreak, vec);

    let mut vec = Vec::new();
    vec.push("goblin_ahah/ahah1.ogg".to_string());
    vec.push("goblin_ahah/ahah2.ogg".to_string());
    vec.push("goblin_ahah/ahah3.ogg".to_string());
    vec.push("goblin_ahah/haha1.ogg".to_string());
    vec.push("goblin_ahah/haha2.ogg".to_string());
    vec.push("goblin_ahah/haha3.ogg".to_string());
    vec.push("goblin_ahah/ooh1.ogg".to_string());
    vec.push("goblin_ahah/ooh2.ogg".to_string());
    c.sfx.insert(GoblinAhah, vec);

    let mut vec = Vec::new();
    vec.push("monsters/rat/rat1.ogg".to_string());
    vec.push("monsters/rat/rat2.ogg".to_string());
    vec.push("monsters/rat/rat3.ogg".to_string());
    c.sfx.insert(EnterRat, vec);

    let mut vec = Vec::new();
    vec.push("monsters/little_monster/little1.ogg".to_string());
    vec.push("monsters/little_monster/little2.ogg".to_string());
    vec.push("monsters/little_monster/little3.ogg".to_string());
    c.sfx.insert(EnterLittleMonster, vec);

    let mut vec = Vec::new();
    vec.push("monsters/big_monster/big1.ogg".to_string());
    vec.push("monsters/big_monster/big2.ogg".to_string());
    vec.push("monsters/big_monster/big3.ogg".to_string());
    c.sfx.insert(EnterBigMonster, vec);

    let mut vec = Vec::new();
    vec.push("monsters/skeleton/skeleton1.ogg".to_string());
    vec.push("monsters/skeleton/skeleton2.ogg".to_string());
    vec.push("monsters/skeleton/skeleton3.ogg".to_string());
    c.sfx.insert(EnterSkeleton, vec);

    let mut vec = Vec::new();
    vec.push("monsters/zombie/zombie1.ogg".to_string());
    vec.push("monsters/zombie/zombie2.ogg".to_string());
    c.sfx.insert(EnterZombie, vec);

    let mut vec = Vec::new();
    vec.push("slash_hit/hit1.ogg".to_string());
    vec.push("slash_hit/hit2.ogg".to_string());
    vec.push("slash_hit/hit3.ogg".to_string());
    c.sfx.insert(SlashHit, vec);

    let mut vec = Vec::new();
    vec.push("sword_clang/clang1.ogg".to_string());
    vec.push("sword_clang/clang2.ogg".to_string());
    vec.push("sword_clang/clang3.ogg".to_string());
    c.sfx.insert(SwordClang, vec);

    let mut vec = Vec::new();
    vec.push("water_dripping/drip1.ogg".to_string());
    c.sfx.insert(WaterDripping, vec);

    // ==============================================================
    // ================= Fonts
    // ==============================================================

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
