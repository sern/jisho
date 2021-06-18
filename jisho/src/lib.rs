use lazy_static::lazy_static;
use pyo3::prelude::*;
use romkan::Romkan;
use serde::Deserialize;
// embedding makes compilation extremely slow and takes up gigantic (up to 20GB) amount of RAM
// pub const JP_BINARY: &[u8] = include_bytes!("../jp.bc");
// pub const JP_CN_BINARY: &[u8] = include_bytes!("../jp-cn.bc");
// pub const JP_EN_BINARY: &[u8] = include_bytes!("../jp-en.bc");
// lazy_static! {
//     pub static ref JP: Vec<Entry> = bincode::deserialize(JP_BINARY).unwrap();
//     pub static ref JP_CN: Vec<Entry> = bincode::deserialize(JP_CN_BINARY).unwrap();
//     pub static ref JP_EN: Vec<Entry> = bincode::deserialize(JP_EN_BINARY).unwrap();
// }

#[derive(Deserialize, Debug, Clone)]
pub struct Entry {
    pub hiragana: String,
    pub kanjis: Vec<String>,
    pub definition: String,
}

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

pub const JP_NAME: &'static str = "Japanese";
pub const JP_CN_NAME: &'static str = "Japanese-Chinese";
pub const JP_EN_NAME: &'static str = "Japanese-English";

pub struct Searcher {}

#[derive(Default)]
pub struct Dictionary(u8);

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn all() -> Self {
        Self(JP_FLAG | JP_CN_FLAG | JP_EN_FLAG)
    }
    pub fn set_jp(self) -> Self {
        Self(self.0 | JP_FLAG)
    }
    pub fn set_jp_cn(self) -> Self {
        Self(self.0 | JP_CN_FLAG)
    }
    pub fn set_jp_en(self) -> Self {
        Self(self.0 | JP_EN_FLAG)
    }
    pub fn jp(&self) -> bool {
        self.0 & JP_FLAG > 0
    }
    pub fn jp_cn(&self) -> bool {
        self.0 & JP_CN_FLAG > 0
    }
    pub fn jp_en(&self) -> bool {
        self.0 & JP_EN_FLAG > 0
    }
}

impl From<u8> for Dictionary {
    fn from(flags: u8) -> Self {
        Self(flags)
    }
}

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

#[derive(Default)]
pub struct SearchResult {
    pub jp: Vec<&'static Entry>,
    pub jp_cn: Vec<&'static Entry>,
    pub jp_en: Vec<&'static Entry>,
}

#[derive(Default)]
pub struct SearchResultSingle {
    pub jp: Option<&'static Entry>,
    pub jp_cn: Option<&'static Entry>,
    pub jp_en: Option<&'static Entry>,
}

impl SearchResultSingle {
    pub fn hinshi(&self) -> Vec<String> {
        use libxml::parser::Parser;
        use libxml::tree::Node;
        fn process_hinshi(x_xdh: &Node) -> String {
            let pos = x_xdh.findnodes(".//span[@class='pos']").unwrap_or_default();
            if pos.len() == 2 {
                return "形動".to_owned();
            }
            let hinshi = pos[0].get_content();
            if &hinshi == "名" {
                let sy = pos[0]
                    .findnodes("./..//span[@class='sy']")
                    .unwrap_or_default();
                if sy.len() == 1 && sy[0].get_content().trim() == "スル" {
                    return "名・スル".to_owned();
                }
            }
            if hinshi.chars().next().unwrap() == '動' {
                return "動".to_owned();
            }
            hinshi
        }

        let entry = Parser::default()
            .parse_string(&self.jp.as_ref().unwrap().definition)
            .unwrap();
        let entry = entry.as_node();
        entry
            .findnodes("//span[contains(@class, 'se1')]/span[contains(@class, 'x_xdh')]")
            .unwrap_or_default()
            .into_iter()
            .map(|node| process_hinshi(&node))
            .collect()
    }
}

pub fn search_exact_interactive(query: &str) -> SearchResultSingle {
    fn _search_exact_interactive(
        q: &str,
        dictionary: &'static [Entry],
        name: &'static str,
    ) -> Option<&'static Entry> {
        let q = standardize_input(q);
        println!("Now searching {} in the {} dictionary...", q, name);
        let stdin = std::io::stdin();
        let mut ans = String::new();
        for entry in dictionary.iter() {
            if entry.hiragana == q || entry.kanjis.iter().any(|x| x == &q) {
                println!("{}|{:?}", entry.hiragana, entry.kanjis);

                stdin.read_line(&mut ans).unwrap();
                if &ans == "\n" {
                    return Some(entry);
                }
                ans.clear();
            }
        }
        println!("Cannot find a match of {} in the {} dictionary. Hit enter to skip or search for another keyword.", q, name);
        stdin.read_line(&mut ans).unwrap();
        match &ans[..ans.len() - 1] {
            // -1 to remove newline
            "" => None,
            q => _search_exact_interactive(&q, dictionary, name),
        }
    }
    SearchResultSingle {
        jp: _search_exact_interactive(query, &JP, JP_NAME),
        jp_cn: _search_exact_interactive(query, &JP_CN, JP_CN_NAME),
        jp_en: _search_exact_interactive(query, &JP_EN, JP_EN_NAME),
    }
}

pub fn standardize_input(input: &str) -> String {
    if input.chars().all(|c| c.is_ascii_alphabetic()) {
        input.to_hiragana()
    } else {
        input.to_owned()
    }
}

pub fn search_exact<D: Into<Dictionary>>(query: &str, dictionaries: D) -> SearchResult {
    _search(query, dictionaries, |entry, query| entry == query)
}

pub fn search_exact_all(query: &str) -> SearchResult {
    search_exact(query, Dictionary::all())
}

pub fn search_starts_with<D: Into<Dictionary>>(query: &str, dictionaries: D) -> SearchResult {
    _search(query, dictionaries, |entry, query| entry.starts_with(query))
}

pub fn search_starts_with_all(query: &str) -> SearchResult {
    search_starts_with(query, Dictionary::all())
}

pub fn search_contains<D: Into<Dictionary>>(query: &str, dictionaries: D) -> SearchResult {
    _search(&query, dictionaries, |entry, query| entry.contains(query))
}

pub fn search_contains_all(query: &str) -> SearchResult {
    search_contains(query, Dictionary::all())
}

fn _search<D: Into<Dictionary>, F: Fn(&str, &str) -> bool>(
    query: &str,
    dictionaries: D,
    predicate: F,
) -> SearchResult {
    fn search_fn<C: Fn(&str, &str) -> bool>(
        query: &str,
        dictionary: &'static [Entry],
        predicate: &C,
    ) -> Vec<&'static Entry> {
        let mut res = Vec::new();
        for entry in dictionary.iter() {
            if predicate(&entry.hiragana, query) || entry.kanjis.iter().any(|x| predicate(x, query))
            {
                res.push(entry);
            }
        }
        res
    }

    let dictionaries = dictionaries.into();
    let mut res = SearchResult::default();

    if dictionaries.jp() {
        res.jp = search_fn(&query, &JP, &predicate);
    }
    if dictionaries.jp_cn() {
        res.jp_cn = search_fn(&query, &JP_CN, &predicate);
    }
    if dictionaries.jp_en() {
        res.jp_en = search_fn(&query, &JP_EN, &predicate)
    }
    res
}
