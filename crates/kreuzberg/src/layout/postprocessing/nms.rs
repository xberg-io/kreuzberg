use crate::layout::types::LayoutDetection;

/// Standard greedy Non-Maximum Suppression.
///
/// Sorts detections by confidence (descending), then iteratively removes
/// detections that have IoU > `iou_threshold` with any higher-confidence detection.
///
/// This is required for YOLO models. RT-DETR is NMS-free.
pub(crate) fn greedy_nms(mut detections: Vec<LayoutDetection>, iou_threshold: f32) -> Vec<LayoutDetection> {
    detections = LayoutDetection::sort_by_confidence_desc(detections);

    let n = detections.len();
    let mut keep = vec![true; n];

    for i in 0..n {
        if !keep[i] {
            continue;
        }
        for j in (i + 1)..n {
            if !keep[j] {
                continue;
            }
            if detections[i].bbox.iou(&detections[j].bbox) > iou_threshold {
                keep[j] = false;
            }
        }
    }

    let mut idx = 0;
    detections.retain(|_| {
        let k = keep[idx];
        idx += 1;
        k
    });

    detections
}
