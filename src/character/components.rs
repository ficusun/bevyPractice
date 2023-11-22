use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct TargetVec(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct SkillCooldownTime(pub Timer);

#[derive(Component, Default)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct IsBot;

#[derive(Bundle, Default)]
pub struct PlayerBundel {
    pub health: Health,
    pub speed: Speed,
    pub target_vec: TargetVec,
    // pub is_player: IsPlayer,
}

// #[derive(Bundle, Default)]
// pub struct BotBundel {
//     pub health: Health,
//     pub speed: Speed,
//     pub target_vec: TargetVec,
//     pub is_bot: IsBot,
// }

#[derive(Component, Default)]
pub struct SkillQuen {
    pub run: [Option<Skills>; 4],
}

/// Skills - enum with skill and angle where to do it
/// default is Move
// #[derive(Default)]
pub enum Skills {
    // #[default]
    Move(f32),
    Attack(f32),
    Flip(f32),
    Teleport(f32),
}

impl Default for Skills {
    fn default() -> Self {
        Self::Move(0.)
    }
}
