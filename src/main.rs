// Is this a comment?

use bevy::{prelude::*, reflect::Tuple};
use tts::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

struct GreetTimer {
    timer: Timer,
}

fn words() -> Vec<RobotWord> {
    vec![
        RobotWord {
            keycode: KeyCode::Q,
            label: "THE".to_string(),
            pronunciation: "thee".to_string(),
        },
        RobotWord {
            keycode: KeyCode::W,
            label: "BE".to_string(),
            pronunciation: "bee".to_string(),
        },
    ]
}
struct RobotWord {
    keycode: KeyCode,
    label: String,
    pronunciation: String,
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer {
            timer: Timer::from_seconds(2.0, true),
        })
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}

fn main() {
    let my_string = "Hemlo I am String";
    println!("Hello, world! {}", my_string);

    App::new()
        .init_resource::<Mytts>()
        .add_startup_system(setup_text)
        .add_system(text_color_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system(keyboard_tts)
        .run();
}

fn hello_world() {
    println!("hello world 2!");
}

#[derive(Component)]
struct ColorText;

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Robot\nTalkin!",
                TextStyle {
                    font: asset_server.load("font/aliee13.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ColorText);
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Kevin".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bob".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Press Button To Speak!".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.timer.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

fn keyboard_tts(
    mut tts: ResMut<Mytts>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    for mut text in query.iter_mut() {
        for robotword in words() {
            if keyboard_input.just_released(robotword.keycode) {
                info!("{} just released", robotword.label);
                text.sections[0].value = robotword.label;

                tts.0.speak(robotword.pronunciation, true).unwrap();
            }
        }
        // if keyboard_input.just_released(KeyCode::Q) {
        //     info!("'the' just released");
        //     text.sections[0].value = "THE".to_string();

        //     tts.0.speak("thee", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::W) {
        //     info!("'be' just released");
        //     text.sections[0].value = "BE".to_string();

        //     tts.0.speak("bee", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::E) {
        //     info!("'to' just released");
        //     text.sections[0].value = "TO".to_string();

        //     tts.0.speak("two", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::R) {
        //     info!("'of' just released");
        //     text.sections[0].value = "OF".to_string();

        //     tts.0.speak("of", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::T) {
        //     info!("'and' just released");
        //     text.sections[0].value = "AND".to_string();

        //     tts.0.speak("and", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::Y) {
        //     info!("'a' just released");
        //     text.sections[0].value = "A".to_string();

        //     tts.0.speak("A", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::U) {
        //     info!("'in' just released");
        //     text.sections[0].value = "IN".to_string();

        //     tts.0.speak("inn", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::I) {
        //     info!("'that' just released");
        //     text.sections[0].value = "THAT".to_string();

        //     tts.0.speak("that", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::O) {
        //     info!("'have' just released");
        //     text.sections[0].value = "HAVE".to_string();

        //     tts.0.speak("have", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::P) {
        //     info!("'I' just released");
        //     text.sections[0].value = "I".to_string();

        //     tts.0.speak("eye", true).unwrap();
        // }
        // if keyboard_input.just_released(KeyCode::A) {
        //     info!("'ahuuhuueueueueh' just released");
        //     text.sections[0].value = "ahuuhuuueuueuuuuuuhhh".to_string();

        //     tts.0.speak("ahuuhuuueuueuuuuuuhhh", true).unwrap();
        // }
    }
}

impl FromWorld for Mytts {
    fn from_world(world: &mut World) -> Self {
        Mytts(Tts::default().unwrap())
    }
}

struct Mytts(Tts);
