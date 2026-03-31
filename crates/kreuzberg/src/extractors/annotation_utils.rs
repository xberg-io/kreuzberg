use crate::types::document_structure::TextAnnotation;

/// Adjust annotation byte offsets after trimming leading whitespace from text.
/// Filters out annotations that fall outside the trimmed text bounds.
pub fn adjust_annotations_for_trim(
    mut annotations: Vec<TextAnnotation>,
    raw_text: &str,
    trimmed_text: &str,
) -> Vec<TextAnnotation> {
    let trim_offset = raw_text.len() - raw_text.trim_start().len();
    let trimmed_len = trimmed_text.len() as u32;
    if trim_offset == 0 {
        // No trimming needed — filter invalid annotations in-place to avoid reallocation.
        annotations.retain(|a| a.start < a.end && a.end <= trimmed_len);
        return annotations;
    }
    let offset = trim_offset as u32;
    annotations
        .into_iter()
        .map(|mut a| {
            a.start = a.start.saturating_sub(offset);
            a.end = a.end.saturating_sub(offset);
            a
        })
        .filter(|a| a.start < a.end && a.end <= trimmed_len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::document_structure::AnnotationKind;

    fn ann(start: u32, end: u32) -> TextAnnotation {
        TextAnnotation {
            start,
            end,
            kind: AnnotationKind::Bold,
        }
    }

    #[test]
    fn no_trimming_needed() {
        let raw = "hello world";
        let trimmed = "hello world";
        let annotations = vec![ann(0, 5)];
        let result = adjust_annotations_for_trim(annotations, raw, trimmed);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 0);
        assert_eq!(result[0].end, 5);
    }

    #[test]
    fn leading_whitespace_trim() {
        let raw = "   hello world";
        let trimmed = "hello world";
        // Annotation at bytes 3..8 in raw -> 0..5 in trimmed
        let annotations = vec![ann(3, 8)];
        let result = adjust_annotations_for_trim(annotations, raw, trimmed);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 0);
        assert_eq!(result[0].end, 5);
    }

    #[test]
    fn annotation_fully_in_trimmed_region() {
        let raw = "     hello";
        let trimmed = "hello";
        // Annotation at bytes 0..3 is entirely in the trimmed whitespace region
        let annotations = vec![ann(0, 3)];
        let result = adjust_annotations_for_trim(annotations, raw, trimmed);
        assert!(result.is_empty(), "annotation in trimmed region should be removed");
    }

    #[test]
    fn annotation_spanning_trim_boundary() {
        let raw = "  hello";
        let trimmed = "hello";
        // Annotation at bytes 1..4 spans the trim boundary (offset=2)
        // After adjustment: start = saturating_sub(1,2) = 0, end = saturating_sub(4,2) = 2
        let annotations = vec![ann(1, 4)];
        let result = adjust_annotations_for_trim(annotations, raw, trimmed);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 0);
        assert_eq!(result[0].end, 2);
    }

    #[test]
    fn empty_annotation_removed() {
        let raw = "hello";
        let trimmed = "hello";
        // start == end -> empty annotation, should be filtered out
        let annotations = vec![ann(2, 2)];
        let result = adjust_annotations_for_trim(annotations, raw, trimmed);
        assert!(result.is_empty(), "empty annotation (start==end) should be removed");
    }
}
