//! Other language stopwords.
//!
//! Includes: Arabic (ar), Hebrew (he), Turkish (tr), Persian (fa),
//! Kurdish (ku), Armenian (hy), Estonian (et), Basque (eu),
//! Breton (br), Esperanto (eo), Finnish (fi), Irish (ga),
//! Hungarian (hu), Indonesian (id), Latin (la), Lithuanian (lt),
//! Latvian (lv), Malay (ms), Tagalog (tl), Greek (el),
//! Hausa (ha), Swahili (sw), Yoruba (yo), Zulu (zu),
//! Somali (so), Sesotho (st).

use ahash::{AHashMap, AHashSet};

/// Macro to generate embedded stopwords for other languages.
macro_rules! embed_stopwords {
    ($map:expr, $($lang:literal),* $(,)?) => {
        $(
            {
                const JSON: &str = include_str!(concat!("../../../stopwords/", $lang, "_stopwords.json"));
                match serde_json::from_str::<Vec<String>>(JSON) {
                    Ok(words) => {
                        let set: AHashSet<String> = words.into_iter().collect();
                        $map.insert($lang.to_string(), set);
                    }
                    Err(e) => {
                        panic!(
                            "Failed to parse embedded stopwords for language '{}': {}. \
                            This indicates corrupted or malformed JSON in the embedded stopwords data. \
                            Please report this issue at https://github.com/kreuzberg-dev/kreuzberg/issues",
                            $lang, e
                        );
                    }
                }
            }
        )*
    };
}

/// Load other language stopwords into the provided map.
pub(in crate::stopwords) fn load_stopwords(map: &mut AHashMap<String, AHashSet<String>>) {
    embed_stopwords!(
        map, "ar", "he", "tr", "fa", "ku", "hy", "et", "eu", "br", "eo", "fi", "ga", "hu", "id", "la", "lt", "lv",
        "ms", "tl", "el", "ha", "sw", "yo", "zu", "so", "st"
    );
}
