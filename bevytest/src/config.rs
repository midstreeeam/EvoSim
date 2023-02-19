use bevy::prelude::*;

// ui config
pub const TIME_STEP: f32 = 1.0 / 60.0;

// physical config
pub const GRAVITY: Vec2 = Vec2{x:0.0, y:0.0};
pub const LINEAR_DAMPING: f32 = 0.5;
pub const ANGULAR_DAMPING: f32 = 0.3;


// blob config
pub const BLOB_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const BLOB_DENSITY: f32 = 2.0;
pub const BLOB_RADIUS: f32 = 30.0;
pub const BLOB_SIZE: Vec3 = Vec3::new(BLOB_RADIUS, BLOB_RADIUS, 0.);


// wall config
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const WALL_THICKNESS: f32 = 10.0;
// half width
pub const WALL_X: f32 = 450.;
// half height
pub const WALL_Y: f32 = 300.;


// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
pub const PADDLE_PADDING: f32 = 10.0;





pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);