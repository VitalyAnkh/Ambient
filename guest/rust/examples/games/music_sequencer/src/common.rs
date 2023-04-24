#![allow(dead_code)]

use ambient_api::prelude::*;

pub const SOUNDS: [(&str, &str); 8] = [
    ("Kick Drum", "assets/BD2500.ogg"),
    ("Snare Drum", "assets/SD7550.ogg"),
    ("Closed Hihat", "assets/CH.ogg"),
    ("Open Hihat", "assets/OH75.ogg"),
    ("Low Conga", "assets/LC00.ogg"),
    ("Mid Conga", "assets/MC00.ogg"),
    ("High Tom", "assets/HT75.ogg"),
    ("Mid Tom", "assets/MT75.ogg"),
];

pub const BEAT_COUNT: usize = 16;
pub const NOTE_COUNT: usize = SOUNDS.len() * BEAT_COUNT;
pub const BEATS_PER_MINUTE: usize = 120;
pub const SECONDS_PER_BEAT: f32 = 60.0 / BEATS_PER_MINUTE as f32;
pub const SECONDS_PER_NOTE: f32 = SECONDS_PER_BEAT / 4.0;

pub fn hsv_to_rgb([h, s, v]: &[f32; 3]) -> Vec3 {
    let c = v * s;
    let p = (h / 60.) % 6.;
    let x = c * (1.0 - ((p % 2.) - 1.).abs());
    let m = Vec3::ONE * (v - c);

    m + match p.trunc() as i32 {
        0 => vec3(c, x, 0.),
        1 => vec3(x, c, 0.),
        2 => vec3(0., c, x),
        3 => vec3(0., x, c),
        4 => vec3(x, 0., c),
        5 => vec3(c, 0., x),
        _ => vec3(0., 0., 0.),
    }
}