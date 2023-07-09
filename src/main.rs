use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::UiPlugin;

mod ui;

pub const CAMERA_SPEED: f32 = 300.0;

#[derive(Resource, Reflect)]
pub struct Tilemap(pub Handle<TextureAtlas>);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState{
    #[default]
    Loading,
    Running
}

fn main(){
    let mut app = App::new();

    app
        .add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(load_tilemap)
        .add_plugin(UiPlugin)
        .add_startup_system(spawn_cam)
        .add_system(move_cam)
        .run()
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_cam(mut commands: Commands){ 

    commands.spawn((Camera2dBundle{
        transform: Transform::from_xyz(0.,0., 0.),
        ..default()
    }, MainCamera));
}

fn move_cam(mut cam_query: Query<&mut Transform, With<MainCamera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    let mut cam = cam_query.get_single_mut().unwrap();
    if keyboard.pressed(KeyCode::A) {
        cam.translation.x -=  CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        cam.translation.x +=  CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        cam.translation.y +=  CAMERA_SPEED * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        cam.translation.y -=  CAMERA_SPEED * time.delta_seconds();
    }
        
}

fn load_tilemap(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let image = assets.load("tilemap_packed.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(32.0), 16, 16, None, None );
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(Tilemap(atlas_handle));
}
