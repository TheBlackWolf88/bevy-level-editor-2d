use bevy::prelude::*;

use crate::{Tilemap, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App, game_state: Res<State<GameState>>) {
        if GameState::Running {
            app.add_startup_system(setup);
        }
            .
    }
}

fn setup(mut commands: Commands, textures: Res<Tilemap>){
    commands.spawn(NodeBundle{
        style: Style {
            size: Size::width(Val::Percent(100.0)),
            justify_content: JustifyContent::End,
            ..default()
        },
        ..default()
        
    }).with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    size: Size::width(Val::Px(200.0)),
                    overflow: Overflow::Hidden,
                    ..default()
                },
                background_color: Color::rgb(0.55, 0.10, 0.25).into(),
                ..default()
            }    
        );
    }); 
}

