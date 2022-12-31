
pub mod koala {
    use serde::{Deserialize, Serialize};
    use serde_with;
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Note{
        pub chance: f32,
        pub length: u32, //16384 seems to be 1 bar in 4/4, ie. 1024 for quarter 
        pub num: u32,
        pub pan: f32,
        pub pitch: f32,
        pub start: f32,
        pub time_offset: u32,
        pub vel: f32
    }
    #[serde_with::serde_as]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Pattern {
        #[serde_as(as = "serde_with::DefaultOnError")]
        pub notes: Vec<Note>
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Sequence {
        pub pattern: Pattern
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SequenceFile{
        pub beats_per_bar: u32,
        pub bpm: f32,
        pub curr_sequence_id: u32,
        pub quantize_division: u32,
        pub quantizing: bool,
        pub sequences: Vec<Sequence>,
        pub swing: f32
    }
    
}