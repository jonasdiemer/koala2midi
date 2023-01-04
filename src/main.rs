use std::fs;
use std::path::PathBuf;
use std::io::Read;
use clap::Parser;


use koala2midi::{koala, midi_export};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input file, can be either a .koala file or a sequence.json file
   input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    
    let ext = args.input_file.extension().expect("invalid extension");
    let mut midi_file_path = args.input_file.clone();
    midi_file_path.set_extension("mid");
    let mut sequence_data = String::from("");

    if ext == "json" {
        sequence_data = fs::read_to_string(args.input_file)
        .expect("Unable to read file");

    } else if ext == "koala" {
        let file = fs::File::open(args.input_file).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        let mut sequence_file = archive.by_name("sequence.json").unwrap();
        sequence_file.read_to_string(&mut sequence_data).unwrap();
    }

    let seq_file: koala::SequenceFile = serde_json::from_str(&sequence_data)
        .expect("JSON does not have correct format.");
    
    midi_export::koala_sequence_to_midi(seq_file, midi_file_path);
}