use super::note2midi;

pub const BPM: f64 = 113.0;
pub const DATA: [(u32, f64); 30] = [
    (note2midi("Ab4"), 0.25),
    (note2midi("Bb4"), 0.25),
    (note2midi("Db5"), 0.25),
    (note2midi("Bb4"), 0.25),
    // m. 1
    (note2midi("F5"), 0.75),
    (note2midi("F5"), 0.75),
    (note2midi("Eb5"), 0.5),
    (0, 1.0),
    (note2midi("Ab4"), 0.25),
    (note2midi("Bb4"), 0.25),
    (note2midi("C5"), 0.25),
    (note2midi("Ab4"), 0.25),
    // m. 2
    (note2midi("Eb5"), 0.75),
    (note2midi("Eb5"), 0.75),
    (note2midi("Db5"), 1.0),
    (note2midi("Bb4"), 0.5),
    (note2midi("Ab4"), 0.25),
    (note2midi("Bb4"), 0.25),
    (note2midi("Db5"), 0.25),
    (note2midi("Ab4"), 0.25),
    // m. 3
    (note2midi("Db5"), 0.1),
    (note2midi("Eb5"), 0.5),
    (note2midi("C5"), 0.75),
    (note2midi("Bb4"), 0.25),
    (note2midi("Ab4"), 0.5),
    (0, 0.5),
    (note2midi("Ab4"), 0.5),
    // m. 4
    (note2midi("Eb5"), 1.0),
    (note2midi("Db5"), 1.5),
    (0, 0.5)
];