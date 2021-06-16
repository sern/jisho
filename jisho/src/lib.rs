use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde::Deserialize;

pub const JP_BINARY: &[u8] = include_bytes!("../jp.bc");
pub const JP_CN_BINARY: &[u8] = include_bytes!("../jp-cn.bc");
pub const JP_EN_BINARY: &[u8] = include_bytes!("../jp-en.bc");

#[pyclass]
#[derive(Deserialize, Debug, Clone)]
pub struct Entry {
    #[pyo3(get)]
    pub hiragana: String,
    #[pyo3(get)]
    pub kanjis: Vec<String>,
    #[pyo3(get)]
    pub definition: String,
}

#[pyclass]
#[derive(Default)]
pub struct SearchResult {
    #[pyo3(get)]
    jp: Vec<Entry>,
    #[pyo3(get)]
    jp_cn: Vec<Entry>,
    #[pyo3(get)]
    jp_en: Vec<Entry>,
}

// impl SearchResult {
//     fn new() -> Self {
//         Self { jp: vec![], jp }
//     }
// }

lazy_static! {
    pub static ref JP: Vec<Entry> = bincode::deserialize(JP_BINARY).unwrap();
    pub static ref JP_CN: Vec<Entry> = bincode::deserialize(JP_CN_BINARY).unwrap();
    pub static ref JP_EN: Vec<Entry> = bincode::deserialize(JP_EN_BINARY).unwrap();
}

#[rustfmt::skip]
pub const JP_FLAG:    u8 = 0b00000001;
pub const JP_CN_FLAG: u8 = 0b00000010;
pub const JP_EN_FLAG: u8 = 0b00000100;

#[rustfmt::skip]
pub const SEARCH_EXACT:    u8 = 0b00000001;
#[rustfmt::skip]
pub const SEARCH_START:    u8 = 0b00000010;
pub const SEARCH_CONTAINS: u8 = 0b00000100;

// #[pyclass]
// pub struct Jisho {}

// pub trait Jisho {
//     fn entries(self) -> &'static [Entry];
// }

// #[pyclass]
// pub struct JpJisho {}

// impl Jisho for JpJisho {
//     fn entries(self) -> &'static [Entry] {
//         &JP
//     }
// }
// #[pyclass]
// pub struct JpCnJisho {}

// impl Jisho for JpCnJisho {
//     fn entries(self) -> &'static [Entry] {
//         &JP_CN
//     }
// }
// #[pyclass]
// pub struct JpEnJisho {}

// #[pymethods]
// impl Jisho for JpEnJisho {
//     fn entries(self) -> &'static [Entry] {
//         &JP_EN
//     }
// }

#[pyfunction]
fn search_starts_with(query: String, dictionaries: u8) -> SearchResult {
    fn _search_starts_with(query: &str, dictionary: &[Entry]) -> Vec<Entry> {
        let mut res = Vec::new();
        for entry in dictionary.iter() {
            if entry.hiragana.starts_with(&query)
                || entry.kanjis.iter().any(|x| x.starts_with(&query))
            {
                res.push(entry.clone());
            }
        }
        res
    }
    _search(&query, dictionaries, _search_starts_with)
}

#[pyfunction]
fn search_exact(query: String, dictionaries: u8) -> SearchResult {
    fn _search_exact(query: &str, dictionary: &[Entry]) -> Vec<Entry> {
        let mut res = Vec::new();
        for entry in dictionary.iter() {
            if entry.hiragana == *query || entry.kanjis.iter().any(|x| x == &query) {
                res.push(entry.clone());
            }
        }
        res
    }
    _search(&query, dictionaries, _search_exact)
}

fn _search<F: Fn(&str, &[Entry]) -> Vec<Entry>>(
    query: &str,
    dictionaries: u8,
    search_fn: F,
) -> SearchResult {
    let mut res = SearchResult::default();

    if JP_FLAG & dictionaries != 0 {
        res.jp = search_fn(&query, &JP);
    }
    if JP_CN_FLAG & dictionaries != 0 {
        res.jp_cn = search_fn(&query, &JP_CN);
    }
    if JP_EN_FLAG & dictionaries != 0 {
        res.jp_en = search_fn(&query, &JP_EN)
    }
    res
}

// #[pyfunction]
// fn search_all_contains(query: String) -> Vec<Entry> {
//     let mut res = Vec::new();
//     let mut contains = Vec::new();
//     for entry in DAIJIRIN.iter() {
//         if entry.hiragana.starts_with(&query) || entry.kanjis.iter().any(|x| x.starts_with(&query))
//         {
//             res.push(entry.clone());
//         } else if entry.hiragana.contains(&query) || entry.kanjis.iter().any(|x| x.contains(&query))
//         {
//             contains.push(entry.clone());
//         }
//     }
//     res.extend_from_slice(&contains);
//     res
// }

#[pymodule]
fn jisho(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Entry>()?;
    m.add_function(wrap_pyfunction!(search_starts_with, m)?)
        .unwrap();
    m.add_function(wrap_pyfunction!(search_exact, m)?).unwrap();
    // #[pyfn(m, "search_interactive")]
    // fn search_interactive(query: String) -> Option<Entry> {
    //     let stdin = io::stdin();
    //     for entry in DAIJIRIN.iter() {
    //         if entry.hiragana.starts_with(&query)
    //             || entry.kanjis.iter().any(|x| x.starts_with(&query))
    //         {
    //             println!("{}|{:?}", entry.hiragana, entry.kanjis);
    //             let mut ans = String::new();
    //             stdin.read_line(&mut ans).unwrap();
    //             if &ans == "\n" {
    //                 return Some(entry.clone());
    //             }
    //         }
    //     }
    //     None
    // }
    Ok(())
}
