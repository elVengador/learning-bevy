use bevy::prelude::*;
// use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, (setup, load_sprites))
        .add_systems(Update, (animate_sprite,keyboard_input_system))
        .run()
}

fn setup(
    mut commands: Commands,
    // rpg_sprite_handles: Res<RpgSpriteFolder>,
    asset_server: Res<AssetServer>,
    // loaded_folders: Res<Assets<LoadedFolder>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut textures: ResMut<Assets<Image>>,
) {
    // commands.spawn(Camera2dBundle::default());
    // // text
    // commands.spawn(TextBundle::from_section(
    //     "Hold 'Left' or 'Right' to change the line width",
    //     TextStyle {
    //         font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //         font_size: 24.,
    //         color: Color::WHITE,
    //     },
    // ));

    commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("background.png"),
    //     ..default()
    // });
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Animation {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct Player;

fn load_sprites(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // assets from: https://clembod.itch.io/warrior-free-animation-set
    let warrior_sprite_sheet: Handle<Image> = assets.load("warrior-sheet.png");
    let atlas = TextureAtlas::from_grid(
        warrior_sprite_sheet,
        Vec2::new(69.0, 44.0),
        6,
        17,
        Some(Vec2::new(0.0,0.0)),
        Some(Vec2::new(0.0,0.0)),
    );
    let atlas_handle = texture_atlases.add(atlas);
    let idle_animation = Animation { first: 0, last: 100 };
    // let position = PlayerTransform {  };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(idle_animation.first),
            transform: Transform { translation: Vec3::new(310.0,0.0,0.0),  scale: Vec3::splat(3.0) ,..Default::default()},
            ..default()
        },
        idle_animation,
        // position,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(Player);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Animation, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>,mut query: Query<(&Player,&mut Transform)>,time: Res<Time>) {
    // if keyboard_input.pressed(KeyCode::A) {
    //     info!("'A' currently pressed");
    // }


    let (_player,mut transform) =  query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        info!("right {}",transform.translation.x);
        transform.translation.x +=100.0*time.delta_seconds();
        // player_transform.position.x=player_transform.position.x+50.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -=100.0*time.delta_seconds();
        info!("left");
    }

    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y +=100.0*time.delta_seconds();
        info!("up");
    }

    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -=100.0*time.delta_seconds();
        info!("down");
    }
    // for (player,mut player_transform) in  query.single_mut() {
        
    //     // timer.tick(time.delta());
    //     // if timer.just_finished() {
    //     //     sprite.index = if sprite.index == indices.last {
    //     //         indices.first
    //     //     } else {
    //     //         sprite.index + 1
    //     //     };
    //     // }
    // }

    

    // if keyboard_input.just_released(KeyCode::A) {
    //     info!("'A' just released");
    // }
}