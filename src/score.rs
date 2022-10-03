use bevy::prelude::*;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(reset_score));
    }
}

#[derive(Deref, DerefMut)]
pub struct Score(pub u32);

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}
