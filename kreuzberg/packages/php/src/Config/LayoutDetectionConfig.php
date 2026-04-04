<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Layout detection configuration.
 *
 * Controls document layout analysis including confidence filtering,
 * heuristic post-processing, and table structure recognition model selection.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\LayoutDetectionConfig;
 *
 * $layout = new LayoutDetectionConfig(
 *     confidenceThreshold: 0.75,
 *     applyHeuristics: true,
 *     tableModel: 'tatr',
 * );
 * ```
 */
readonly class LayoutDetectionConfig
{
    /**
     * @param float|null $confidenceThreshold Minimum confidence threshold for detected layout
     *                                        regions (0.0-1.0). Regions below this threshold are discarded.
     *                                        Default null (use engine default).
     * @param bool $applyHeuristics Whether to apply heuristic post-processing to refine
     *                              layout regions. Default true.
     * @param string|null $tableModel Table structure recognition model to use.
     *                                Supported values: "tatr", "slanet_wired", "slanet_wireless",
     *                                "slanet_plus", "slanet_auto", "disabled".
     *                                Default null (use engine default, "tatr").
     */
    public function __construct(
        public ?float $confidenceThreshold = null,
        public bool $applyHeuristics = true,
        public ?string $tableModel = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $rawThreshold = $data['confidence_threshold'] ?? null;
        $confidenceThreshold = is_numeric($rawThreshold) ? (float) $rawThreshold : null;
        $applyHeuristics = isset($data['apply_heuristics']) ? (bool) $data['apply_heuristics'] : true;
        $rawTableModel = $data['table_model'] ?? null;
        $tableModel = is_string($rawTableModel) ? $rawTableModel : null;

        return new self(
            confidenceThreshold: $confidenceThreshold,
            applyHeuristics: $applyHeuristics,
            tableModel: $tableModel,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [
            'apply_heuristics' => $this->applyHeuristics,
        ];

        if ($this->confidenceThreshold !== null) {
            $result['confidence_threshold'] = $this->confidenceThreshold;
        }

        if ($this->tableModel !== null) {
            $result['table_model'] = $this->tableModel;
        }

        return $result;
    }
}
