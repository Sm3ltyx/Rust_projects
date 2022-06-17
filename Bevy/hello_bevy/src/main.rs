use bevy::{prelude::*, text::Text2dBounds};

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

//SYSTEMS

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn setup (mut commands: Commands, asset_server: Res<AssetServer>){
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Demonstrate text wrapping
    let box_size = Size::new(780.0, 360.0);
    let box_position = Vec2::new(0.0, 0.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(box_size.width, box_size.height)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.0)),
        ..default()
    });
    let text_alignment_top_left = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | Hello BEVY! | ",
            text_style,
            text_alignment_top_left,
        ),
        text_2d_bounds: Text2dBounds {
            // Wrap text in the rectangle
            size: box_size,
        },
        // We align text to the top-left, so this transform is the top-left corner of our text. The
        // box is centered at box_position, so it is necessary to move by half of the box size to
        // keep the text in the box.
        transform: Transform::from_xyz(
            box_position.x - box_size.width / 2.0,
            box_position.y + box_size.height / 2.0,
            1.0,
        ),
        ..default()
    });
}

//COMPONENTS & ENTITIES

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

//PLUGIN

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, false)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}
