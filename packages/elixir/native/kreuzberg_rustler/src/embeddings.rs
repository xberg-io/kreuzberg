use crate::atoms;
use crate::config::parse_embedding_config;
use kreuzberg::embed_texts;
use rustler::{Encoder, Env, NifResult, Term};

#[rustler::nif(schedule = "DirtyCpu")]
pub fn embed<'a>(env: Env<'a>, texts: Vec<String>, config_term: Term<'a>) -> NifResult<Term<'a>> {
    let config = match parse_embedding_config(env, config_term) {
        Ok(c) => c,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    match embed_texts(&texts, &config) {
        Ok(result) => Ok((atoms::ok(), result).encode(env)),
        Err(e) => {
            let error_atom = if e.to_string().to_lowercase().contains("embedding") {
                atoms::embedding_error()
            } else {
                atoms::error()
            };
            Ok((error_atom, format!("Embedding failed: {}", e)).encode(env))
        }
    }
}
