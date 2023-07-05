use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_graphics)
        .add_systems((
            display_override,
            toggle_override,
            change_scale_factor
        ));
    }
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));
}


/// Set the title of the window to the current override
fn display_override(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.title = format!(
        "Scale override: {:?}",
        window.resolution.scale_factor_override()
    );
}

/// This system toggles scale factor overrides when enter is pressed
fn toggle_override(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    if input.just_pressed(KeyCode::Return) {
        let scale_factor_override = window.resolution.scale_factor_override();
        window
            .resolution
            .set_scale_factor_override(scale_factor_override.xor(Some(1.0)));
    }
}

/// This system changes the scale factor override when up or down is pressed
fn change_scale_factor(input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    let scale_factor_override = window.resolution.scale_factor_override();
    if input.just_pressed(KeyCode::Up) {
        window
            .resolution
            .set_scale_factor_override(scale_factor_override.map(|n| n + 1.0));
    } else if input.just_pressed(KeyCode::Down) {
        window
            .resolution
            .set_scale_factor_override(scale_factor_override.map(|n| (n - 1.0).max(1.0)));
    }
}