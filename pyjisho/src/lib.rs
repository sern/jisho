use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Entry {
    #[pyo3(get)]
    pub hiragana: String,
    #[pyo3(get)]
    pub kanjis: Vec<String>,
    #[pyo3(get)]
    pub definition: String,
}

impl From<jisho::Entry> for Entry {
    fn from(inp: jisho::Entry) -> Entry {
        Entry {
            hiragana: inp.hiragana,
            kanjis: inp.kanjis,
            definition: inp.definition,
        }
    }
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

impl From<jisho::SearchResult> for SearchResult {
    fn from(inp: jisho::SearchResult) -> Self {
        Self {
            jp: inp.jp.iter().map(|&e| Entry::from(e.clone())).collect(),
            jp_cn: inp.jp_cn.iter().map(|&e| Entry::from(e.clone())).collect(),
            jp_en: inp.jp_en.iter().map(|&e| Entry::from(e.clone())).collect(),
        }
    }
}

#[pyclass]
#[derive(Default)]
pub struct SearchResultSingle {
    #[pyo3(get)]
    hinshi: Vec<String>,
    #[pyo3(get)]
    jp: Option<Entry>,
    #[pyo3(get)]
    jp_cn: Option<Entry>,
    #[pyo3(get)]
    jp_en: Option<Entry>,
}

impl From<jisho::SearchResultSingle> for SearchResultSingle {
    fn from(inp: jisho::SearchResultSingle) -> Self {
        Self {
            hinshi: inp.hinshi(),
            jp: inp.jp.map(|e| e.clone().into()),
            jp_cn: inp.jp_cn.map(|e| e.clone().into()),
            jp_en: inp.jp_en.map(|e| e.clone().into()),
        }
    }
}

// #[pyfunction]
// pub fn search_contains(query: String, dictionaries: u8) -> SearchResult {
//     jisho::search_contains(&query, dictionaries).into()
// }

#[pymodule]
fn jisho(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Entry>()?;
    m.add_class::<SearchResult>()?;
    m.add_class::<SearchResultSingle>()?;
    #[pyfn(m, "search_exact")]
    fn search_exact(query: String, dictionaries: u8) -> SearchResult {
        jisho::search_exact(&query, dictionaries).into()
    }
    #[pyfn(m, "search_exact_all")]
    fn search_exact_all(query: String) -> SearchResult {
        jisho::search_exact_all(&query).into()
    }
    #[pyfn(m, "search_starts_with")]
    fn search_starts_with(query: String, dictionaries: u8) -> SearchResult {
        jisho::search_starts_with(&query, dictionaries).into()
    }
    #[pyfn(m, "search_starts_with_all")]
    fn search_starts_with_all(query: String) -> SearchResult {
        jisho::search_starts_with_all(&query).into()
    }
    #[pyfn(m, "search_contains")]
    fn search_contains(query: String, dictionaries: u8) -> SearchResult {
        jisho::search_contains(&query, dictionaries).into()
    }
    #[pyfn(m, "search_contains_all")]
    fn search_contains_all(query: String) -> SearchResult {
        jisho::search_contains_all(&query).into()
    }
    #[pyfn(m, "search_exact_interactive")]
    fn search_exact_interactive(query: String) -> Option<SearchResultSingle> {
        jisho::search_exact_interactive(&query).map(|r| r.into())
    }
    Ok(())
}
