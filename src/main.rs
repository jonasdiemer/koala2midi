use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Note{
    chance: f32,
    length: i32,
    num: u32,
    pan: f32,
    pitch: f32,
    start: f32,
    time_offset: usize,
    vel: f32
}
#[derive(Serialize, Deserialize, Debug)]
struct Pattern {
    notes: Option<Vec<Note>>
}
#[derive(Serialize, Deserialize, Debug)]
struct Sequence {
    pattern: Pattern
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SequenceFile{
    beats_per_bar: u32,
    bpm: f32,
    curr_sequence_id: u32,
    quantize_division: u32,
    quantizing: bool,
    sequences: Vec<Sequence>,
    swing: f32
}


fn main() {
    let data = fs::read_to_string("./sequence.json")
        .expect("Unable to read file");

    let seq_file: SequenceFile = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
    

    dbg!(seq_file);
}