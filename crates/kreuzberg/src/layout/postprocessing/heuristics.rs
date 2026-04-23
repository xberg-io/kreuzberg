use crate::layout::types::{LayoutClass, LayoutDetection};

/// Per-class confidence thresholds from Docling's layout_postprocessor.py.
///
/// Specialized classes (section_header, title, code, form, key_value_region)
/// use a lower threshold (0.45) since they are rarer and more valuable to detect.
/// Common classes use 0.50.
fn class_threshold(class: LayoutClass) -> f32 {
    match class {
        LayoutClass::SectionHeader
        | LayoutClass::Title
        | LayoutClass::Code
        | LayoutClass::Form
        | LayoutClass::KeyValueRegion => 0.45,
        _ => 0.50,
    }
}

/// Apply Docling-style postprocessing heuristics to raw detections.
///
/// This implements the key heuristics from `docling/utils/layout_postprocessor.py`:
/// 1. Per-class confidence thresholds
/// 2. Full-page picture removal (>90% page area)
/// 3. Overlap resolution (IoU > 0.8 or containment > 0.8)
/// 4. Cross-type overlap handling (KVR vs Table)
pub(crate) fn apply_heuristics(
    mut detections: Vec<LayoutDetection>,
    page_width: f32,
    page_height: f32,
) -> Vec<LayoutDetection> {
    // 1. Apply per-class confidence thresholds.
    detections.retain(|d| d.confidence >= class_threshold(d.class));

    // 2. Remove full-page pictures (>90% of page area).
    detections.retain(|d| {
        if d.class == LayoutClass::Picture {
            d.bbox.page_coverage(page_width, page_height) < 0.9
        } else {
            true
        }
    });

    // 2b. Demote tiny Table/Picture false positives to Text.
    //     If a Table or Picture covers <3% of page area AND has confidence <0.7,
    //     it is likely a false positive that would suppress body text.
    for d in detections.iter_mut() {
        if matches!(d.class, LayoutClass::Table | LayoutClass::Picture)
            && d.bbox.page_coverage(page_width, page_height) < 0.03
            && d.confidence < 0.7
        {
            d.class = LayoutClass::Text;
        }
    }

    // 3. Overlap resolution — iterative (up to 3 passes).
    for _ in 0..3 {
        let prev_len = detections.len();
        resolve_overlaps(&mut detections);
        if detections.len() == prev_len {
            break;
        }
    }

    // 4. Cross-type overlap: remove KVR if 90%+ overlapping with Table and conf_diff < 0.1.
    resolve_kvr_table_overlap(&mut detections);
    detections
}

/// Resolve overlapping detections.
///
/// When two detections overlap significantly (IoU > 0.8 or containment > 0.8),
/// remove the lower-confidence one, with special label-preference rules.
fn resolve_overlaps(detections: &mut Vec<LayoutDetection>) {
    let n = detections.len();
    let mut remove = vec![false; n];

    for i in 0..n {
        if remove[i] {
            continue;
        }
        for j in (i + 1)..n {
            if remove[j] {
                continue;
            }

            let iou = detections[i].bbox.iou(&detections[j].bbox);
            let containment_i_of_j = detections[i].bbox.containment_of(&detections[j].bbox);
            let containment_j_of_i = detections[j].bbox.containment_of(&detections[i].bbox);

            // Skip if no significant overlap.
            if iou < 0.8 && containment_i_of_j < 0.8 && containment_j_of_i < 0.8 {
                continue;
            }

            // Determine which to remove using label-specific preference rules.
            let remove_idx = pick_removal(&detections[i], &detections[j], containment_i_of_j);
            if remove_idx == 0 {
                remove[i] = true;
            } else {
                remove[j] = true;
            }
        }
    }

    let mut idx = 0;
    detections.retain(|_| {
        let k = !remove[idx];
        idx += 1;
        k
    });
}

/// Determine which of two overlapping detections to remove.
/// Returns 0 to remove `a`, 1 to remove `b`.
fn pick_removal(a: &LayoutDetection, b: &LayoutDetection, containment_a_of_b: f32) -> usize {
    // ListItem preferred over Text when similar area (±20%).
    if a.class == LayoutClass::ListItem && b.class == LayoutClass::Text {
        let area_ratio = a.bbox.area() / b.bbox.area().max(1e-6);
        if (0.8..=1.2).contains(&area_ratio) {
            return 1; // remove Text
        }
    }
    if b.class == LayoutClass::ListItem && a.class == LayoutClass::Text {
        let area_ratio = b.bbox.area() / a.bbox.area().max(1e-6);
        if (0.8..=1.2).contains(&area_ratio) {
            return 0; // remove Text
        }
    }

    // Code preferred when other is 80%+ contained.
    if a.class == LayoutClass::Code && containment_a_of_b > 0.8 {
        return 1; // remove b
    }
    if b.class == LayoutClass::Code {
        let containment_b_of_a = b.bbox.containment_of(&a.bbox);
        if containment_b_of_a > 0.8 {
            return 0; // remove a
        }
    }

    // Text preferred over Table/Picture when Text has equal or higher confidence.
    // This prevents low-confidence Table/Picture detections from suppressing body text.
    if a.class == LayoutClass::Text
        && matches!(b.class, LayoutClass::Table | LayoutClass::Picture)
        && a.confidence >= b.confidence
    {
        return 1; // remove Table/Picture, keep Text
    }
    if b.class == LayoutClass::Text
        && matches!(a.class, LayoutClass::Table | LayoutClass::Picture)
        && b.confidence >= a.confidence
    {
        return 0; // remove Table/Picture, keep Text
    }

    // Default: keep higher confidence.
    if a.confidence >= b.confidence { 1 } else { 0 }
}

/// Remove KeyValueRegion if 90%+ overlapping with Table and confidence difference < 0.1.
fn resolve_kvr_table_overlap(detections: &mut Vec<LayoutDetection>) {
    let n = detections.len();
    let mut remove = vec![false; n];

    for i in 0..n {
        if remove[i] || detections[i].class != LayoutClass::KeyValueRegion {
            continue;
        }
        for j in 0..n {
            if i == j || remove[j] || detections[j].class != LayoutClass::Table {
                continue;
            }
            let overlap = detections[j].bbox.containment_of(&detections[i].bbox);
            let conf_diff = (detections[i].confidence - detections[j].confidence).abs();
            if overlap > 0.9 && conf_diff < 0.1 {
                remove[i] = true;
                break;
            }
        }
    }

    let mut idx = 0;
    detections.retain(|_| {
        let k = !remove[idx];
        idx += 1;
        k
    });
}
