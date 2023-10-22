use crate::scoreboard::Scoreboard;
use bevy::prelude::*;
pub fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
        let mut text = query.single_mut();
        text.sections[1].value = score.score.to_string();
}