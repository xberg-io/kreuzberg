//! DAG-based reading order sorting for layout regions.
//!
//! Builds a predecessor/successor directed acyclic graph from spatial relationships
//! between layout regions, then resolves reading order via DFS traversal.
//! Handles arbitrary column layouts (2+, asymmetric, mixed).
//!
//! Inspired by Docling's rule-based ReadingOrderPredictor, adapted to work with
//! Kreuzberg's layout region types and PDF coordinate system (y=0 at bottom).

use super::LayoutRegion;
use crate::pdf::markdown::constants::REGION_SAME_ROW_FRACTION;
use crate::pdf::markdown::types::LayoutHintClass;

/// Epsilon for "strictly above" comparison (in points).
const STRICTLY_ABOVE_EPS: f32 = 1e-3;

/// X-padding when querying for predecessor/successor candidates (in points).
const HORIZONTAL_OVERLAP_PADDING: f32 = 0.1;

/// Maximum horizontal dilation as a fraction of page width.
const MAX_DILATION_FRACTION: f32 = 0.15;

/// Minimum number of body regions to use DAG ordering (below this, use simple sort).
const MIN_REGIONS_FOR_DAG: usize = 4;

/// Sort regions in reading order using a DAG-based algorithm.
///
/// 1. Separates PAGE_HEADER/PAGE_FOOTER from body regions (ordered independently).
/// 2. Builds a predecessor/successor DAG based on vertical adjacency with horizontal overlap.
/// 3. Applies horizontal dilation to strengthen column links.
/// 4. Resolves reading order via DFS with upward ancestor traversal.
/// 5. Falls back to Y-quantized sort for simple pages (< 4 body regions).
pub(in crate::pdf::markdown) fn order_regions_reading_order(regions: &mut [LayoutRegion], page_height: f32) {
    if regions.len() < 2 {
        return;
    }

    // Separate furniture from body regions
    let mut header_indices: Vec<usize> = Vec::new();
    let mut footer_indices: Vec<usize> = Vec::new();
    let mut body_indices: Vec<usize> = Vec::new();

    for (i, r) in regions.iter().enumerate() {
        match r.hint.class {
            LayoutHintClass::PageHeader => header_indices.push(i),
            LayoutHintClass::PageFooter => footer_indices.push(i),
            _ => body_indices.push(i),
        }
    }

    // Sort headers/footers by Y (top-first for headers, bottom-first for footers)
    header_indices.sort_by(|&a, &b| {
        let (_, a_bot, _, a_top) = regions[a].bbox();
        let (_, b_bot, _, b_top) = regions[b].bbox();
        let a_cy = (a_top + a_bot) / 2.0;
        let b_cy = (b_top + b_bot) / 2.0;
        b_cy.total_cmp(&a_cy)
    });
    footer_indices.sort_by(|&a, &b| {
        let (_, a_bot, _, a_top) = regions[a].bbox();
        let (_, b_bot, _, b_top) = regions[b].bbox();
        let a_cy = (a_top + a_bot) / 2.0;
        let b_cy = (b_top + b_bot) / 2.0;
        b_cy.total_cmp(&a_cy)
    });

    // Order body regions
    let body_order = if body_indices.len() < MIN_REGIONS_FOR_DAG {
        tracing::trace!(
            body_regions = body_indices.len(),
            "reading order: simple Y-sort (below DAG threshold)"
        );
        sort_simple(&body_indices, regions, page_height)
    } else {
        tracing::trace!(body_regions = body_indices.len(), "reading order: DAG-based");
        dag_reading_order(&body_indices, regions, page_height)
    };

    // Assemble final order: headers, body, footers
    let mut final_order: Vec<usize> = Vec::with_capacity(regions.len());
    final_order.extend(&header_indices);
    final_order.extend(&body_order);
    final_order.extend(&footer_indices);

    reorder_by_indices(regions, &final_order);
}

/// Simple Y-quantized sort for pages with few regions.
fn sort_simple(indices: &[usize], regions: &[LayoutRegion], page_height: f32) -> Vec<usize> {
    let y_tolerance = page_height * REGION_SAME_ROW_FRACTION;
    let mut sorted = indices.to_vec();
    if y_tolerance > 0.0 {
        sorted.sort_by(|&a, &b| {
            let (a_left, a_bot, _, a_top) = regions[a].bbox();
            let (b_left, b_bot, _, b_top) = regions[b].bbox();
            let a_cy = (a_top + a_bot) / 2.0;
            let b_cy = (b_top + b_bot) / 2.0;
            let a_row = (a_cy / y_tolerance).round() as i64;
            let b_row = (b_cy / y_tolerance).round() as i64;
            b_row.cmp(&a_row).then_with(|| a_left.total_cmp(&b_left))
        });
    }
    sorted
}

/// DAG-based reading order for body regions.
///
/// Builds a predecessor/successor graph, applies horizontal dilation to
/// strengthen column links, then resolves via DFS with upward traversal.
fn dag_reading_order(body_indices: &[usize], regions: &[LayoutRegion], _page_height: f32) -> Vec<usize> {
    let n = body_indices.len();

    // Extract bboxes for the body regions (in PDF coords: y=0 at bottom)
    // Use effective bbox (merged union if available) for correct spatial ordering.
    let bboxes: Vec<RegionBbox> = body_indices
        .iter()
        .map(|&idx| {
            let (left, bottom, right, top) = regions[idx].bbox();
            RegionBbox {
                left,
                bottom,
                right,
                top,
            }
        })
        .collect();

    // Compute page width for dilation threshold
    let page_left = bboxes.iter().map(|b| b.left).fold(f32::MAX, f32::min);
    let page_right = bboxes.iter().map(|b| b.right).fold(f32::MIN, f32::max);
    let page_width = page_right - page_left;

    // Phase 1: Build initial DAG on original bboxes
    let (up_map, dn_map) = build_dag(&bboxes);

    // Phase 2: Horizontal dilation — expand bboxes toward predecessors/successors,
    // then re-build DAG on dilated bboxes
    let dilated_bboxes = apply_dilation(&bboxes, &up_map, &dn_map, page_width);
    let (_up_map_d, dn_map_d) = build_dag(&dilated_bboxes);

    // Phase 3: Find head nodes (no predecessors in the dilated DAG)
    let up_map_d = {
        let mut up: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, succs) in dn_map_d.iter().enumerate() {
            for &j in succs {
                up[j].push(i);
            }
        }
        up
    };

    let mut heads: Vec<usize> = (0..n).filter(|&i| up_map_d[i].is_empty()).collect();

    // Sort heads: top-to-bottom (higher Y first in PDF coords), then left-to-right
    heads.sort_by(|&a, &b| {
        let a_cy = (bboxes[a].top + bboxes[a].bottom) / 2.0;
        let b_cy = (bboxes[b].top + bboxes[b].bottom) / 2.0;
        b_cy.total_cmp(&a_cy)
            .then_with(|| bboxes[a].left.total_cmp(&bboxes[b].left))
    });

    // Sort each node's successors list
    let mut sorted_dn: Vec<Vec<usize>> = dn_map_d;
    for succs in &mut sorted_dn {
        succs.sort_by(|&a, &b| {
            let a_cy = (bboxes[a].top + bboxes[a].bottom) / 2.0;
            let b_cy = (bboxes[b].top + bboxes[b].bottom) / 2.0;
            b_cy.total_cmp(&a_cy)
                .then_with(|| bboxes[a].left.total_cmp(&bboxes[b].left))
        });
    }

    // Phase 4: DFS traversal with upward ancestor resolution
    let order = dfs_reading_order(&heads, &sorted_dn, &up_map_d, n);

    // Map local indices back to region indices
    order.iter().map(|&local| body_indices[local]).collect()
}

/// Bounding box for DAG computation.
#[derive(Clone)]
struct RegionBbox {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

/// Build predecessor/successor DAG from bounding box spatial relationships.
///
/// For each pair (i, j) where i is strictly above j and they have horizontal overlap,
/// checks that no interrupting element exists in the corridor between them.
fn build_dag(bboxes: &[RegionBbox]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = bboxes.len();
    let mut up_map: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut dn_map: Vec<Vec<usize>> = vec![Vec::new(); n];

    for j in 0..n {
        for i in 0..n {
            if i == j {
                continue;
            }

            // i must be strictly above j (in PDF coords: i.bottom > j.top)
            if bboxes[i].bottom + STRICTLY_ABOVE_EPS <= bboxes[j].top {
                continue;
            }

            // Must have horizontal overlap (with small padding)
            let x_overlap = (bboxes[i].right + HORIZONTAL_OVERLAP_PADDING)
                .min(bboxes[j].right + HORIZONTAL_OVERLAP_PADDING)
                - (bboxes[i].left - HORIZONTAL_OVERLAP_PADDING).max(bboxes[j].left - HORIZONTAL_OVERLAP_PADDING);
            if x_overlap <= 0.0 {
                continue;
            }

            // Check no interrupting element between i and j
            if has_interruption(bboxes, i, j) {
                continue;
            }

            // i is a predecessor of j
            if !up_map[j].contains(&i) {
                up_map[j].push(i);
            }
            if !dn_map[i].contains(&j) {
                dn_map[i].push(j);
            }
        }
    }

    (up_map, dn_map)
}

/// Check if any element w interrupts the vertical corridor between i (above) and j (below).
fn has_interruption(bboxes: &[RegionBbox], i: usize, j: usize) -> bool {
    // Corridor: vertical range = between j.top and i.bottom
    let corridor_bottom = bboxes[j].top; // top of j (lower element)
    let corridor_top = bboxes[i].bottom; // bottom of i (upper element)

    if corridor_bottom >= corridor_top {
        return false; // No vertical space between them
    }

    for (w, bbox_w) in bboxes.iter().enumerate() {
        if w == i || w == j {
            continue;
        }

        // w must horizontally overlap i OR j individually (Docling semantics)
        let overlaps_i = bbox_w.right > bboxes[i].left && bbox_w.left < bboxes[i].right;
        let overlaps_j = bbox_w.right > bboxes[j].left && bbox_w.left < bboxes[j].right;

        if !(overlaps_i || overlaps_j) {
            continue;
        }

        // w must be strictly below i and strictly above j (i.e., w is between them vertically)
        let below_i = bboxes[i].bottom + STRICTLY_ABOVE_EPS > bbox_w.top;
        let above_j = bbox_w.bottom + STRICTLY_ABOVE_EPS > bboxes[j].top;

        if below_i && above_j {
            return true;
        }
    }

    false
}

/// Apply horizontal dilation to bboxes to strengthen column links.
///
/// For each element, expands its x-extent toward its predecessor/successor's extent,
/// but only if the expansion is less than 15% of page width per side and doesn't
/// overlap any other element.
fn apply_dilation(
    bboxes: &[RegionBbox],
    up_map: &[Vec<usize>],
    dn_map: &[Vec<usize>],
    page_width: f32,
) -> Vec<RegionBbox> {
    let threshold = page_width * MAX_DILATION_FRACTION;
    let mut dilated = bboxes.to_vec();

    for i in 0..bboxes.len() {
        let mut target_left = bboxes[i].left;
        let mut target_right = bboxes[i].right;

        // Expand toward first predecessor only (Docling semantics)
        if let Some(&pred) = up_map[i].first() {
            target_left = target_left.min(bboxes[pred].left);
            target_right = target_right.max(bboxes[pred].right);
        }

        // Expand toward first successor only (Docling semantics)
        if let Some(&succ) = dn_map[i].first() {
            target_left = target_left.min(bboxes[succ].left);
            target_right = target_right.max(bboxes[succ].right);
        }

        // Check dilation amount doesn't exceed threshold on either side
        let left_dilation = bboxes[i].left - target_left;
        let right_dilation = target_right - bboxes[i].right;

        if left_dilation > threshold || right_dilation > threshold {
            continue; // Skip dilation for this element
        }

        // Check dilated box doesn't overlap any other original element
        let candidate = RegionBbox {
            left: target_left,
            bottom: bboxes[i].bottom,
            right: target_right,
            top: bboxes[i].top,
        };

        let overlaps_other = bboxes.iter().enumerate().any(|(j, other)| {
            if j == i {
                return false;
            }
            // 2D overlap check
            candidate.left < other.right
                && candidate.right > other.left
                && candidate.bottom < other.top
                && candidate.top > other.bottom
        });

        if !overlaps_other {
            dilated[i] = candidate;
        }
    }

    dilated
}

/// DFS traversal with upward ancestor resolution.
///
/// Before visiting a node, walks up the predecessor chain to find the topmost
/// unvisited ancestor, ensuring column predecessors are visited first.
fn dfs_reading_order(heads: &[usize], dn_map: &[Vec<usize>], up_map: &[Vec<usize>], n: usize) -> Vec<usize> {
    let mut visited = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for &head in heads {
        if visited[head] {
            continue;
        }
        dfs_visit(head, dn_map, up_map, &mut visited, &mut order);
    }

    // Catch any orphan nodes not reachable from heads
    for i in 0..n {
        if !visited[i] {
            dfs_visit(i, dn_map, up_map, &mut visited, &mut order);
        }
    }

    order
}

/// DFS visit with explicit stack (no recursion) and upward ancestor resolution.
fn dfs_visit(start: usize, dn_map: &[Vec<usize>], up_map: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
    // Find topmost unvisited ancestor of start
    let root = find_topmost_unvisited(start, up_map, visited);
    if visited[root] {
        return;
    }

    // Explicit DFS stack: (successors_list, offset_into_list)
    visited[root] = true;
    order.push(root);

    let mut stack: Vec<(&[usize], usize)> = vec![(dn_map[root].as_slice(), 0)];

    while let Some(frame) = stack.last_mut() {
        if frame.1 >= frame.0.len() {
            stack.pop();
            continue;
        }

        let candidate = frame.0[frame.1];
        frame.1 += 1;

        // Walk up to find topmost unvisited ancestor
        let target = find_topmost_unvisited(candidate, up_map, visited);

        if !visited[target] {
            visited[target] = true;
            order.push(target);
            stack.push((dn_map[target].as_slice(), 0));
        }
    }
}

/// Walk up the predecessor chain to find the topmost unvisited ancestor.
fn find_topmost_unvisited(start: usize, up_map: &[Vec<usize>], visited: &[bool]) -> usize {
    let mut current = start;
    let mut changed = true;

    // Iterate until no unvisited predecessors found (with cycle protection)
    while changed {
        changed = false;
        for &pred in &up_map[current] {
            if !visited[pred] {
                current = pred;
                changed = true;
                break;
            }
        }
    }

    current
}

/// Reorder a slice in-place according to the given index order.
fn reorder_by_indices<T>(slice: &mut [T], order: &[usize]) {
    debug_assert_eq!(slice.len(), order.len());
    let mut perm: Vec<usize> = vec![0; slice.len()];
    for (new_pos, &old_pos) in order.iter().enumerate() {
        perm[new_pos] = old_pos;
    }

    // Apply permutation in-place using cycle decomposition.
    //
    // perm[new_pos] = old_pos means result[new_pos] should hold what was at old_pos.
    // We follow each cycle: starting from position i, swap(i, perm[i]) to bring the
    // correct element to position i. After each swap perm[i] is marked done (set to i).
    // Repeat for the next position in the chain until we close the cycle (next == i).
    // No final extra swap is needed since the last element in the cycle is already
    // placed by the preceding swaps.
    let mut visited = vec![false; slice.len()];
    for i in 0..slice.len() {
        if visited[i] || perm[i] == i {
            visited[i] = true;
            continue;
        }
        let mut j = i;
        loop {
            let next = perm[j];
            visited[j] = true;
            if next == i {
                // Cycle is closed; element at j already sits in its correct slot.
                break;
            }
            slice.swap(j, next);
            perm[j] = j;
            j = next;
        }
    }
}
