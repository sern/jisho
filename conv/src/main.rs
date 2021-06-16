use bincode;
use conv::{EntryJP, EntryRawJP};

use std::fs::File;
use std::io::prelude::*;

const JP_JSON_PATH: &str = "../extract/jp.json";
const JP_CN_JSON_PATH: &str = "../extract/jp-cn.json";
const JP_EN_JSON_PATH: &str = "../extract/jp-en.json";

const JP_OUT_PATH: &str = "../jisho/jp.bc";
const JP_CN_OUT_PATH: &str = "../jisho/jp-cn.bc";
const JP_EN_OUT_PATH: &str = "../jisho/jp-en.bc";

fn main() {
    for (inp, out) in [JP_JSON_PATH, JP_CN_JSON_PATH, JP_EN_JSON_PATH]
        .iter()
        .zip([JP_OUT_PATH, JP_CN_OUT_PATH, JP_EN_OUT_PATH].iter())
    {
        let mut f = File::open(inp).unwrap();
        let mut txt = String::new();
        f.read_to_string(&mut txt).unwrap();
        let json: Vec<EntryRawJP> = serde_json::from_str(&txt).unwrap();
        let x: Vec<EntryJP> = json
            .into_iter()
            .map(|e| EntryJP {
                hiragana: e.0,
                kanjis: e.1,
                definition: e.2,
            })
            .collect();
        let data: Vec<u8> = bincode::serialize(&x).unwrap();
        let mut f = File::create(out).unwrap();
        f.write_all(&data).unwrap();
    }
}
