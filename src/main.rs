use std::fs;
use std::path::PathBuf;
use clap::Parser;
use koala2midi::{koala, midi_export};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   sequence_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    
    let mut midi_file_path = args.sequence_file.clone();
    midi_file_path.set_extension("mid");
    
    let data = fs::read_to_string(args.sequence_file)
        .expect("Unable to read file");

    let seq_file: koala::SequenceFile = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
    

    midi_export::koala_sequence_to_midi(seq_file, midi_file_path);
}