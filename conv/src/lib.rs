use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryRawJP(pub String, pub Vec<String>, pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryJP {
    pub hiragana: String,
    pub kanjis: Vec<String>,
    pub definition: String,
}

// pub struct EntryCN {
//     pub name: String,
//     pub definition: String,
// }

// pub struct EntryEn {
//     pub name: String,
//     pub definition: String,
// }
