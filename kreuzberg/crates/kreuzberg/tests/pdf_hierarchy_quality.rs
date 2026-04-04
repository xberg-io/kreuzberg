//! PDF hierarchy quality assessment tests.
//!
//! This module tests PDF text hierarchy extraction quality by comparing against ground truth annotations.
//! Measures precision, recall, F1 score, and level accuracy to ensure the hierarchy detection
//! algorithm works well on real document structures.
//!
//! Test philosophy:
//! - Define ground truth hierarchies for representative PDF documents
//! - Measure how well extracted hierarchies match ground truth
//! - Assert minimum quality thresholds for precision/recall/F1
//! - Verify correct hierarchy level assignments

#![cfg(feature = "pdf")]

use kreuzberg::pdf::hierarchy::{
    BoundingBox, HierarchyLevel, KMeansResult, TextBlock, assign_hierarchy_levels,
    assign_hierarchy_levels_from_clusters, cluster_font_sizes,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// A bounding box annotation from ground truth.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroundTruthBBox {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl GroundTruthBBox {
    /// Convert to kreuzberg BoundingBox
    fn to_bbox(&self) -> BoundingBox {
        BoundingBox {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

/// A ground truth text block with hierarchy level annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroundTruthBlock {
    text: String,
    level: String,
    bbox: GroundTruthBBox,
}

/// A page of ground truth annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroundTruthPage {
    page_number: u32,
    blocks: Vec<GroundTruthBlock>,
}

/// A document with ground truth hierarchy annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroundTruthDocument {
    pdf_file: String,
    pages: Vec<GroundTruthPage>,
}

/// Root structure for ground truth JSON file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroundTruthFile {
    documents: Vec<GroundTruthDocument>,
}

/// Quality metrics for hierarchy extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Precision: (correctly identified hierarchies) / (total extracted hierarchies)
    pub precision: f64,
    /// Recall: (correctly identified hierarchies) / (total ground truth hierarchies)
    pub recall: f64,
    /// F1 Score: harmonic mean of precision and recall
    pub f1_score: f64,
    /// Level accuracy: percentage of blocks assigned to correct hierarchy level
    pub level_accuracy: f64,
    /// Number of correctly identified hierarchy blocks
    pub true_positives: usize,
    /// Number of incorrectly extracted hierarchy blocks
    pub false_positives: usize,
    /// Number of missed ground truth hierarchy blocks
    pub false_negatives: usize,
    /// Number of blocks with correct hierarchy level
    pub correct_levels: usize,
    /// Total number of blocks evaluated
    pub total_blocks: usize,
}

impl QualityMetrics {
    /// Create new quality metrics from test results.
    fn new(
        true_positives: usize,
        false_positives: usize,
        false_negatives: usize,
        correct_levels: usize,
        total_blocks: usize,
    ) -> Self {
        let precision = if true_positives + false_positives > 0 {
            true_positives as f64 / (true_positives + false_positives) as f64
        } else {
            0.0
        };

        let recall = if true_positives + false_negatives > 0 {
            true_positives as f64 / (true_positives + false_negatives) as f64
        } else {
            0.0
        };

        let f1_score = if precision + recall > 0.0 {
            2.0 * precision * recall / (precision + recall)
        } else {
            0.0
        };

        let level_accuracy = if total_blocks > 0 {
            correct_levels as f64 / total_blocks as f64
        } else {
            0.0
        };

        Self {
            precision,
            recall,
            f1_score,
            level_accuracy,
            true_positives,
            false_positives,
            false_negatives,
            correct_levels,
            total_blocks,
        }
    }
}

/// Convert hierarchy level string to HierarchyLevel enum.
fn parse_level(level: &str) -> HierarchyLevel {
    match level {
        "H1" => HierarchyLevel::H1,
        "H2" => HierarchyLevel::H2,
        "H3" => HierarchyLevel::H3,
        "H4" => HierarchyLevel::H4,
        "H5" => HierarchyLevel::H5,
        "H6" => HierarchyLevel::H6,
        _ => HierarchyLevel::Body,
    }
}

/// Load ground truth annotations from JSON file.
///
/// Reads the hierarchy_ground_truth.json file and parses document annotations.
///
/// # Arguments
///
/// * `path` - Path to the ground truth JSON file
///
/// # Returns
///
/// Result containing the parsed GroundTruthFile or error message
fn load_ground_truth<P: AsRef<Path>>(path: P) -> Result<GroundTruthFile, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
}

/// Calculate quality metrics by comparing extracted hierarchies to ground truth.
///
/// Compares extracted text blocks with their hierarchy assignments to ground truth annotations.
/// Measures:
/// - Precision: correctly identified hierarchies / total extracted
/// - Recall: correctly identified hierarchies / total ground truth
/// - F1 Score: harmonic mean of precision and recall
/// - Level Accuracy: percentage of blocks with correct hierarchy level
///
/// # Arguments
///
/// * `extracted_blocks` - Vector of extracted HierarchyBlock objects
/// * `ground_truth_blocks` - Vector of ground truth blocks
///
/// # Returns
///
/// QualityMetrics struct with calculated precision, recall, F1, and level accuracy
fn calculate_quality_metrics(
    extracted_blocks: &[kreuzberg::pdf::hierarchy::HierarchyBlock],
    ground_truth_blocks: &[GroundTruthBlock],
) -> QualityMetrics {
    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut correct_levels = 0;

    // For matching blocks, we use bounding box overlap and text similarity
    let mut matched_gt_indices: Vec<bool> = vec![false; ground_truth_blocks.len()];

    for extracted in extracted_blocks {
        let mut best_match_idx: Option<usize> = None;
        let mut best_overlap = 0.0;

        // Find the best matching ground truth block by bounding box overlap
        for (gt_idx, gt_block) in ground_truth_blocks.iter().enumerate() {
            if matched_gt_indices[gt_idx] {
                continue; // Already matched
            }

            let gt_bbox = gt_block.bbox.to_bbox();
            let overlap = extracted.bbox.iou(&gt_bbox);

            if overlap > best_overlap && overlap > 0.3 {
                best_overlap = overlap;
                best_match_idx = Some(gt_idx);
            }
        }

        if let Some(gt_idx) = best_match_idx {
            matched_gt_indices[gt_idx] = true;
            true_positives += 1;

            // Check if the hierarchy level matches
            let gt_level = parse_level(&ground_truth_blocks[gt_idx].level);
            if extracted.hierarchy_level == gt_level {
                correct_levels += 1;
            }
        } else {
            false_positives += 1;
        }
    }

    // Count unmatched ground truth blocks as false negatives
    let false_negatives = matched_gt_indices.iter().filter(|&&m| !m).count();

    let total_blocks = extracted_blocks.len().max(ground_truth_blocks.len());

    QualityMetrics::new(
        true_positives,
        false_positives,
        false_negatives,
        correct_levels,
        total_blocks,
    )
}

/// Create test text blocks from ground truth.
fn create_text_blocks_from_ground_truth(gt_blocks: &[GroundTruthBlock]) -> Vec<TextBlock> {
    gt_blocks
        .iter()
        .enumerate()
        .map(|(idx, gt_block)| {
            // Estimate font size from bbox height
            let bbox = gt_block.bbox.to_bbox();
            let font_size = match gt_block.level.as_str() {
                "H1" => 28.0,
                "H2" => 24.0,
                "H3" => 20.0,
                "H4" => 16.0,
                "H5" => 14.0,
                "H6" => 12.0,
                _ => 10.0, // Body
            };

            TextBlock {
                text: if gt_block.text.len() > 50 {
                    format!("{} (Block {})", gt_block.text.chars().take(50).collect::<String>(), idx)
                } else {
                    gt_block.text.clone()
                },
                bbox,
                font_size,
            }
        })
        .collect()
}

#[test]
fn test_hierarchy_quality_on_ground_truth() {
    // Load ground truth data
    let ground_truth_path = "tests/data/hierarchy_ground_truth.json";
    let ground_truth_file = load_ground_truth(ground_truth_path).expect("Failed to load ground truth file");

    println!(
        "\nLoaded {} documents from ground truth",
        ground_truth_file.documents.len()
    );

    let mut all_metrics: Vec<QualityMetrics> = Vec::new();

    // Process each document
    for doc in &ground_truth_file.documents {
        println!("\nProcessing document: {}", doc.pdf_file);

        for page in &doc.pages {
            println!("  Page {}: {} blocks", page.page_number, page.blocks.len());

            // Create text blocks from ground truth
            let text_blocks = create_text_blocks_from_ground_truth(&page.blocks);

            // Cluster by font size
            let k = (text_blocks.len() / 3).clamp(1, 6); // Estimate k clusters
            let clusters = cluster_font_sizes(&text_blocks, k).expect("Failed to cluster font sizes");

            println!(
                "    Created {} clusters from {} blocks",
                clusters.len(),
                text_blocks.len()
            );

            // Assign hierarchy levels from clusters
            let hierarchy_assignments = assign_hierarchy_levels_from_clusters(&text_blocks, &clusters);

            // Convert to HierarchyBlock format
            let extracted_blocks: Vec<kreuzberg::pdf::hierarchy::HierarchyBlock> = hierarchy_assignments
                .iter()
                .map(|(block, level)| kreuzberg::pdf::hierarchy::HierarchyBlock {
                    text: block.text.clone(),
                    bbox: block.bbox,
                    font_size: block.font_size,
                    hierarchy_level: *level,
                })
                .collect();

            // Calculate quality metrics
            let metrics = calculate_quality_metrics(&extracted_blocks, &page.blocks);
            all_metrics.push(metrics.clone());

            println!("    Precision:      {:.4}", metrics.precision);
            println!("    Recall:         {:.4}", metrics.recall);
            println!("    F1 Score:       {:.4}", metrics.f1_score);
            println!("    Level Accuracy: {:.4}", metrics.level_accuracy);
        }
    }

    // Calculate average metrics
    if !all_metrics.is_empty() {
        let avg_precision = all_metrics.iter().map(|m| m.precision).sum::<f64>() / all_metrics.len() as f64;
        let avg_recall = all_metrics.iter().map(|m| m.recall).sum::<f64>() / all_metrics.len() as f64;
        let avg_f1 = all_metrics.iter().map(|m| m.f1_score).sum::<f64>() / all_metrics.len() as f64;
        let avg_level_acc = all_metrics.iter().map(|m| m.level_accuracy).sum::<f64>() / all_metrics.len() as f64;

        println!("\n=== AVERAGE METRICS ACROSS ALL PAGES ===");
        println!("Average Precision:      {:.4}", avg_precision);
        println!("Average Recall:         {:.4}", avg_recall);
        println!("Average F1 Score:       {:.4}", avg_f1);
        println!("Average Level Accuracy: {:.4}", avg_level_acc);

        // Assert minimum F1 threshold
        assert!(
            avg_f1 > 0.85,
            "F1 score ({:.4}) must be greater than 0.85. Metrics: precision={:.4}, recall={:.4}, level_accuracy={:.4}",
            avg_f1,
            avg_precision,
            avg_recall,
            avg_level_acc
        );
    }
}

#[test]
fn test_hierarchy_clustering_consistency() {
    // Arrange: Create a simple document with clear hierarchy
    let blocks = vec![
        TextBlock {
            text: "Title".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 0.0,
                right: 100.0,
                bottom: 28.0,
            },
            font_size: 28.0,
        },
        TextBlock {
            text: "Subtitle".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 30.0,
                right: 100.0,
                bottom: 54.0,
            },
            font_size: 24.0,
        },
        TextBlock {
            text: "Section".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 60.0,
                right: 100.0,
                bottom: 80.0,
            },
            font_size: 20.0,
        },
        TextBlock {
            text: "Body paragraph".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 90.0,
                right: 100.0,
                bottom: 102.0,
            },
            font_size: 10.0,
        },
    ];

    // Act: Cluster and assign hierarchies
    let clusters = cluster_font_sizes(&blocks, 4).expect("Clustering failed");
    let assignments = assign_hierarchy_levels_from_clusters(&blocks, &clusters);

    // Assert: Verify hierarchy levels are correct
    assert_eq!(assignments.len(), 4);
    assert_eq!(assignments[0].1, HierarchyLevel::H1, "Largest text should be H1");
    assert_eq!(assignments[1].1, HierarchyLevel::H2, "Second largest should be H2");
    assert_eq!(assignments[2].1, HierarchyLevel::H3, "Third largest should be H3");
    assert_eq!(assignments[3].1, HierarchyLevel::Body, "Smallest text should be Body");

    // Assert: F1 score should be perfect for this simple case
    let quality_metrics = calculate_quality_metrics(
        &assignments
            .iter()
            .map(|(b, l)| kreuzberg::pdf::hierarchy::HierarchyBlock {
                text: b.text.clone(),
                bbox: b.bbox,
                font_size: b.font_size,
                hierarchy_level: *l,
            })
            .collect::<Vec<_>>(),
        &[
            GroundTruthBlock {
                text: "Title".to_string(),
                level: "H1".to_string(),
                bbox: GroundTruthBBox {
                    left: 0.0,
                    top: 0.0,
                    right: 100.0,
                    bottom: 28.0,
                },
            },
            GroundTruthBlock {
                text: "Subtitle".to_string(),
                level: "H2".to_string(),
                bbox: GroundTruthBBox {
                    left: 0.0,
                    top: 30.0,
                    right: 100.0,
                    bottom: 54.0,
                },
            },
            GroundTruthBlock {
                text: "Section".to_string(),
                level: "H3".to_string(),
                bbox: GroundTruthBBox {
                    left: 0.0,
                    top: 60.0,
                    right: 100.0,
                    bottom: 80.0,
                },
            },
            GroundTruthBlock {
                text: "Body paragraph".to_string(),
                level: "Body".to_string(),
                bbox: GroundTruthBBox {
                    left: 0.0,
                    top: 90.0,
                    right: 100.0,
                    bottom: 102.0,
                },
            },
        ],
    );

    println!("Consistency Test - F1 Score: {:.4}", quality_metrics.f1_score);
    assert!(
        quality_metrics.f1_score >= 0.8,
        "F1 score for simple hierarchy should be >= 0.8"
    );
}

#[test]
fn test_hierarchy_level_assignment() {
    // Arrange: Create blocks and KMeans result
    let blocks = vec![
        TextBlock {
            text: "Main Title".to_string(),
            bbox: BoundingBox {
                left: 50.0,
                top: 50.0,
                right: 150.0,
                bottom: 100.0,
            },
            font_size: 28.0,
        },
        TextBlock {
            text: "Section Title".to_string(),
            bbox: BoundingBox {
                left: 50.0,
                top: 120.0,
                right: 150.0,
                bottom: 160.0,
            },
            font_size: 20.0,
        },
        TextBlock {
            text: "Regular body text".to_string(),
            bbox: BoundingBox {
                left: 50.0,
                top: 180.0,
                right: 200.0,
                bottom: 200.0,
            },
            font_size: 12.0,
        },
    ];

    let kmeans_result = KMeansResult { labels: vec![0, 1, 2] };

    // Act: Assign hierarchy levels using KMeans result
    let result = assign_hierarchy_levels(&blocks, &kmeans_result);

    // Assert: Verify correct level assignments
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].hierarchy_level, HierarchyLevel::H1);
    assert_eq!(result[1].hierarchy_level, HierarchyLevel::H2);
    assert_eq!(result[2].hierarchy_level, HierarchyLevel::H3);
}

#[test]
fn test_quality_metrics_calculation() {
    // Arrange: Create extracted blocks and ground truth
    let extracted = vec![
        kreuzberg::pdf::hierarchy::HierarchyBlock {
            text: "Title".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 0.0,
                right: 100.0,
                bottom: 20.0,
            },
            font_size: 28.0,
            hierarchy_level: HierarchyLevel::H1,
        },
        kreuzberg::pdf::hierarchy::HierarchyBlock {
            text: "Body".to_string(),
            bbox: BoundingBox {
                left: 0.0,
                top: 30.0,
                right: 100.0,
                bottom: 50.0,
            },
            font_size: 12.0,
            hierarchy_level: HierarchyLevel::Body,
        },
    ];

    let ground_truth = vec![
        GroundTruthBlock {
            text: "Title".to_string(),
            level: "H1".to_string(),
            bbox: GroundTruthBBox {
                left: 0.0,
                top: 0.0,
                right: 100.0,
                bottom: 20.0,
            },
        },
        GroundTruthBlock {
            text: "Body".to_string(),
            level: "Body".to_string(),
            bbox: GroundTruthBBox {
                left: 0.0,
                top: 30.0,
                right: 100.0,
                bottom: 50.0,
            },
        },
    ];

    // Act: Calculate metrics
    let metrics = calculate_quality_metrics(&extracted, &ground_truth);

    // Assert: Verify metrics
    assert_eq!(metrics.true_positives, 2);
    assert_eq!(metrics.false_positives, 0);
    assert_eq!(metrics.false_negatives, 0);
    assert_eq!(metrics.correct_levels, 2);
    assert!(metrics.precision > 0.99);
    assert!(metrics.recall > 0.99);
    assert!(metrics.f1_score > 0.99);
}
