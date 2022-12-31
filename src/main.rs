use std::fs;

use koala2midi::{koala, midi_export};


fn main() {
    let data = fs::read_to_string("./sequence.json")
        .expect("Unable to read file");

    let seq_file: koala::SequenceFile = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
    

    midi_export::koala_sequence_to_midi(seq_file);
}