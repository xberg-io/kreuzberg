//! ExtractionResult -> R named list conversion and serialization

use crate::helpers::json_to_robj;
use crate::error::to_r_error;
use extendr_api::prelude::*;
use kreuzberg::ExtractionResult;

/// Convert an ExtractionResult to an R named list
pub fn extraction_result_to_list(result: ExtractionResult) -> extendr_api::Result<List> {
    // Serialize to JSON then convert to R objects
    let json_value = serde_json::to_value(&result).map_err(to_r_error)?;
    let robj = json_to_robj(&json_value)?;

    // Convert to list and add class attribute
    let list = List::try_from(robj).map_err(to_r_error)?;
    let mut result_robj = list.into_robj();
    result_robj.set_class(&["kreuzberg_result", "list"]).map_err(to_r_error)?;
    List::try_from(result_robj).map_err(to_r_error)
}
