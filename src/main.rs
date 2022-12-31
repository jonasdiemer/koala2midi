use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
use midi_file::file::{QuartersPerMinute, Track};
use midi_file::MidiFile;

use std::fs;

use koala2midi::koala;

#[derive(Debug)]
struct MidiMessage{
    abs_time: u32,
    on: bool,
    channel: Channel,
    note_number: NoteNumber,
    velocity: Velocity
}


fn note_to_midi_messages(note: koala::Note) -> Vec<MidiMessage> {
    // calculate MIDI note number from pitch, assuming 0 is C1 = 24
    let note_number= NoteNumber::new((24+note.pitch as i8) as u8);
    let channel = Channel::new(note.num as u8);
    let velocity = Velocity::new(note.vel as u8);
    vec![
        MidiMessage {
            abs_time: note.time_offset,
            on: true,
            channel,
            note_number,
            velocity
        },
        MidiMessage {
            abs_time: note.time_offset + note.length,
            on: false,
            channel,
            note_number,
            velocity
        }
    ]
}

/// Returns a vector of midi events
fn koala_sequence_to_midi(sf: koala::SequenceFile) {
    let mut mfile = MidiFile::new();

    // convert sequences to tracks
    for s in sf.sequences {
        // Skip empty sequences
        if s.pattern.notes.is_empty(){
            continue;
        }

        // TODO: Create list of midi events for note on and off (2 events per note)
        //       with absolute times
        //       then sort them by absolute time to properly compute delta times later
        let mut messages: Vec<MidiMessage> = s.pattern.notes
            .into_iter()
            .map(|n| note_to_midi_messages(n))
            .flatten()
            .collect();

        
        messages.sort_by(|a, b| a.abs_time.cmp(&b.abs_time));

        // dbg!(&messages);
        
        // set up track metadata
        let mut track = Track::default();
        //track.set_name("Singer").unwrap();
        //track.set_instrument_name("Alto").unwrap();
        //track.set_general_midi(CH, GeneralMidi::SynthVoice).unwrap();

        // set time signature and tempo
        track
            .push_time_signature(0, sf.beats_per_bar as u8, DurationName::Quarter, Clocks::Quarter)
            .unwrap();
        track.push_tempo(0, QuartersPerMinute::new(sf.bpm as u8)).unwrap();
        
        let mut curr_time = 0;
        for msg in messages{
            // create the note
            // we don't have any rests, all of our lyric and note-on events will be at delta time zero
            
            let delta_time = msg.abs_time - curr_time;
            curr_time = msg.abs_time;


            if msg.on {
                track.push_note_on(
                    delta_time,
                    msg.channel,
                    msg.note_number,
                    msg.velocity).unwrap();
            } else {
                track
                    .push_note_off(
                        delta_time,
                        msg.channel,
                        msg.note_number, 
                        msg.velocity)
                    .unwrap();
            }
        }
        dbg!(&track);
        mfile.push_track(track).unwrap();
    }

    mfile.save("test.mid").unwrap();
}

fn main() {
    let data = fs::read_to_string("./sequence.json")
        .expect("Unable to read file");

    let seq_file: koala::SequenceFile = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");
    

    koala_sequence_to_midi(seq_file);
}