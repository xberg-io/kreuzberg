//! File and bytes extraction functions (sync + async via Tokio)

use crate::config::parse_config;
use crate::error::{kreuzberg_error, to_r_error};
use crate::result::extraction_result_to_list;
use extendr_api::prelude::*;

pub fn extract_file_sync_impl(path: &str, mime_type: Nullable<&str>, config_json: Nullable<&str>) -> extendr_api::Result<List> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let config = parse_config(config_json)?;
        let mime = match mime_type {
            Nullable::NotNull(m) => Some(m),
            Nullable::Null => None,
        };
        let result = kreuzberg::extract_file_sync(path, mime, &config).map_err(kreuzberg_error)?;
        extraction_result_to_list(result)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (path, mime_type, config_json);
        Err("File extraction is not supported on WebAssembly".into())
    }
}

pub fn extract_file_impl(path: &str, mime_type: Nullable<&str>, config_json: Nullable<&str>) -> extendr_api::Result<List> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let config = parse_config(config_json)?;
        let mime = match mime_type {
            Nullable::NotNull(m) => Some(m),
            Nullable::Null => None,
        };
        let runtime = tokio::runtime::Runtime::new().map_err(to_r_error)?;
        let result = runtime
            .block_on(async { kreuzberg::extract_file(path, mime, &config).await })
            .map_err(kreuzberg_error)?;
        extraction_result_to_list(result)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (path, mime_type, config_json);
        Err("Async file extraction is not supported on WebAssembly".into())
    }
}

pub fn extract_bytes_sync_impl(data: Raw, mime_type: &str, config_json: Nullable<&str>) -> extendr_api::Result<List> {
    let config = parse_config(config_json)?;
    let bytes = data.as_slice();
    let result = kreuzberg::extract_bytes_sync(bytes, mime_type, &config).map_err(kreuzberg_error)?;
    extraction_result_to_list(result)
}

pub fn render_pdf_page_impl(path: &str, page_index: i32, dpi: i32) -> extendr_api::Result<Raw> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if page_index < 0 {
            return Err("page_index must be non-negative".into());
        }
        let pdf_bytes = std::fs::read(path).map_err(to_r_error)?;
        let dpi_opt = if dpi <= 0 { None } else { Some(dpi) };
        let png = kreuzberg::pdf::render_pdf_page_to_png(&pdf_bytes, page_index as usize, dpi_opt, None)
            .map_err(to_r_error)?;
        Ok(Raw::from_bytes(&png))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (path, page_index, dpi);
        Err("PDF rendering is not supported on WebAssembly".into())
    }
}

pub fn extract_bytes_impl(data: Raw, mime_type: &str, config_json: Nullable<&str>) -> extendr_api::Result<List> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let config = parse_config(config_json)?;
        let bytes = data.as_slice();
        let runtime = tokio::runtime::Runtime::new().map_err(to_r_error)?;
        let result = runtime
            .block_on(async { kreuzberg::extract_bytes(bytes, mime_type, &config).await })
            .map_err(kreuzberg_error)?;
        extraction_result_to_list(result)
    }
    #[cfg(target_arch = "wasm32")]
    {
        extract_bytes_sync_impl(data, mime_type, config_json)
    }
}
