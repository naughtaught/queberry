use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
struct LanguageInfo {
    name: String,
    code: String,
    code2: String,
}

static LANGUAGES: OnceLock<HashMap<String, LanguageInfo>> = OnceLock::new();

fn get_languages() -> &'static HashMap<String, LanguageInfo> {
    LANGUAGES.get_or_init(|| {
        let languages_json = r#"
        [
            {"name": "English", "code": "en", "code2": "eng"},
            {"name": "Spanish", "code": "es", "code2": "spa"},
            {"name": "French", "code": "fr", "code2": "fre"},
            {"name": "German", "code": "de", "code2": "ger"},
            {"name": "Japanese", "code": "ja", "code2": "jpn"},
            {"name": "Chinese", "code": "zh", "code2": "chi"},
            {"name": "Korean", "code": "ko", "code2": "kor"},
            {"name": "Russian", "code": "ru", "code2": "rus"},
            {"name": "Italian", "code": "it", "code2": "ita"},
            {"name": "Portuguese", "code": "pt", "code2": "por"}
        ]
        "#;

        let languages_list: Vec<LanguageInfo> =
            serde_json::from_str(languages_json).unwrap_or_else(|_| Vec::new());

        let mut map = HashMap::new();
        for lang in languages_list {
            map.insert(lang.code.clone(), lang);
        }
        map
    })
}

pub fn name_to_code(name: &str) -> String {
    let languages = get_languages();
    let name_lower = name.to_lowercase();

    for lang in languages.values() {
        if lang.name.to_lowercase() == name_lower
            || lang.code.to_lowercase() == name_lower
            || lang.code2.to_lowercase() == name_lower
        {
            return lang.code.clone();
        }
    }

    name_lower
}

pub fn normalize_lang_base(s: &str) -> String {
    let mut s = name_to_code(s);
    s = s.to_lowercase();

    s = s.replace("–", "-").replace("—", "-");

    if let Some(idx) = s.find('-') {
        s = s[..idx].to_string();
    }

    s
}

pub fn language_matches(lang1: &str, lang2: &str) -> bool {
    let base1 = normalize_lang_base(lang1);
    let base2 = normalize_lang_base(lang2);

    if base1 == base2 {
        return true;
    }

    let languages = get_languages();
    for lang in languages.values() {
        if base1 == lang.code2.to_lowercase() || base2 == lang.code2.to_lowercase() {
            return true;
        }
    }

    false
}
