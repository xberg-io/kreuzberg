//! Asian language stopwords.
//!
//! Includes: Chinese (zh), Japanese (ja), Korean (ko), Thai (th),
//! Vietnamese (vi), Hindi (hi), Bengali (bn), Gujarati (gu),
//! Kannada (kn), Malayalam (ml), Marathi (mr), Tamil (ta),
//! Telugu (te), Nepali (ne), Sinhala (si), Urdu (ur).

use ahash::{AHashMap, AHashSet};

/// Macro to generate embedded stopwords for Asian languages.
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

/// Load Asian language stopwords into the provided map.
pub(in crate::stopwords) fn load_stopwords(map: &mut AHashMap<String, AHashSet<String>>) {
    embed_stopwords!(
        map, "zh", "ja", "ko", "th", "vi", "hi", "bn", "gu", "kn", "ml", "mr", "ta", "te", "ne", "si", "ur"
    );
}
