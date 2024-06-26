/*
music/badapple.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Tempo and music data for _Bad Apple!!_ ft. Nomico.


use super::note2midi;

pub const BPM: f64 = 138.0;
pub const DATA: [(u32, f64); 107] = [
    // m. 1
    (note2midi("Eb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Eb6"), 0.5),
    (note2midi("Db6"), 0.5),
    // m. 2
    (note2midi("Bb5"), 1.0),
    (note2midi("Eb5"), 1.0),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("F5"), 0.5),
    // m. 3
    (note2midi("Eb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    // m. 4
    (note2midi("F5"), 0.5),
    (note2midi("Eb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Eb5"), 0.5),
    (note2midi("D5"), 0.5),
    (note2midi("F5"), 0.5),
    // m. 5
    (note2midi("Eb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Eb6"), 0.5),
    (note2midi("Db6"), 0.5),
    // m. 6
    (note2midi("Bb5"), 1.0),
    (note2midi("Eb5"), 1.0),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("F5"), 0.5),
    // m. 7
    (note2midi("Eb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    // m. 8
    (note2midi("F5"), 1.0),
    (note2midi("Gb5"), 1.0),
    (note2midi("Ab5"), 1.0),
    (note2midi("Bb5"), 1.0),
    // m. 9
    (note2midi("Db6"), 0.5),
    (note2midi("Eb6"), 0.5),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 0.5),
    // m. 10
    (note2midi("Db6"), 0.5),
    (note2midi("Eb6"), 0.5),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 0.5),
    // m. 11
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Db5"), 0.5),
    (note2midi("Eb5"), 1.0),
    (note2midi("Db5"), 0.5),
    (note2midi("Eb5"), 0.5),
    // m. 12
    (note2midi("F5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 0.5),
    (note2midi("Eb5"), 1.0),
    (note2midi("Bb5"), 0.5),
    (note2midi("Db6"), 0.5),
    // m. 13
    (note2midi("Db6"), 0.5),
    (note2midi("Eb6"), 0.5),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 0.5),
    // m. 14
    (note2midi("Db6"), 0.5),
    (note2midi("Eb6"), 0.5),
    (note2midi("Bb5"), 0.5),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Eb6"), 0.5),
    (note2midi("F6"), 0.5),
    // m. 15
    (note2midi("Gb6"), 0.5),
    (note2midi("F6"), 0.5),
    (note2midi("Eb6"), 0.5),
    (note2midi("Db6"), 0.5),
    (note2midi("Bb5"), 1.0),
    (note2midi("Ab5"), 0.5),
    (note2midi("Bb5"), 0.5),
    // m. 16
    (note2midi("Ab5"), 0.5),
    (note2midi("Gb5"), 0.5),
    (note2midi("F5"), 0.5),
    (note2midi("Db5"), 0.5),
    (note2midi("Eb5"), 1.0),
    (0, 1.0),
];