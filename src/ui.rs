use bevy::prelude::*;

use crate::{Tilemap, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(test.in_schedule(OnEnter(GameState::Running)));
        }
    }



fn test(){
    println!("This is a test startup system");
}


fn setup(mut commands: Commands){
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
                background_color: Color::rgb(0.15, 0.15, 0.17).into(),
                ..default()
            }    
        );
    }); 
}

