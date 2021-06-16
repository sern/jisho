use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

// pub const JP_BINARY: &[u8] = include_bytes!("../jp.bc");
// pub const JP_CN_BINARY: &[u8] = include_bytes!("../jp-cn.bc");
// pub const JP_EN_BINARY: &[u8] = include_bytes!("../jp-en.bc");

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

#[pyclass]
#[derive(Default)]
pub struct SearchResultSingle {
    #[pyo3(get)]
    jp: Option<Entry>,
    #[pyo3(get)]
    jp_cn: Option<Entry>,
    #[pyo3(get)]
    jp_en: Option<Entry>,
}

// impl SearchResult {
//     fn new() -> Self {
//         Self { jp: vec![], jp }
//     }
// }

// lazy_static! {
//     pub static ref JP: Vec<Entry> = {
//         let jp_binary = std::fs::read("raw/jp.data");
//         bincode::deserialize(jp_binary).unwrap()
//     };
//     pub static ref JP_CN: Vec<Entry> = bincode::deserialize(JP_CN_BINARY).unwrap();
//     pub static ref JP_EN: Vec<Entry> = bincode::deserialize(JP_EN_BINARY).unwrap();
// }

lazy_static! {
    pub static ref JP: Vec<Entry> = {
        let binary = std::fs::read("jisho/jp.bc").unwrap();
        bincode::deserialize(&binary).unwrap()
    };
    pub static ref JP_CN: Vec<Entry> = {
        let binary = std::fs::read("jisho/jp-cn.bc").unwrap();
        bincode::deserialize(&binary).unwrap()
    };
    pub static ref JP_EN: Vec<Entry> = {
        let binary = std::fs::read("jisho/jp-en.bc").unwrap();
        bincode::deserialize(&binary).unwrap()
    };
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
    #[pyfn(m, "search_exact_interactive")]
    fn search_exact_interactive(query: Vec<String>) -> SearchResultSingle {
        fn _search_exact_interactive(query: &[String], dictionary: &[Entry]) -> Option<Entry> {
            let stdin = std::io::stdin();
            for entry in dictionary.iter() {
                for q in query {
                    if &entry.hiragana == q || entry.kanjis.iter().any(|x| x == q) {
                        println!("{}|{:?}", entry.hiragana, entry.kanjis);
                        let mut ans = String::new();
                        stdin.read_line(&mut ans).unwrap();
                        if &ans == "\n" {
                            return Some(entry.clone());
                        }
                    }
                }
            }
            None
        }
        SearchResultSingle {
            jp: _search_exact_interactive(&query, &JP),
            jp_cn: _search_exact_interactive(&query, &JP_CN),
            jp_en: _search_exact_interactive(&query, &JP_EN),
        }
    }
    Ok(())
}
