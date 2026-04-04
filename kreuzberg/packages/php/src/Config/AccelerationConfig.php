<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Hardware acceleration configuration for ONNX Runtime models.
 *
 * Configures which execution provider to use for ONNX model inference,
 * enabling hardware acceleration on GPU devices (CUDA, TensorRT, CoreML).
 *
 * @example
 * ```php
 * use Kreuzberg\Config\AccelerationConfig;
 *
 * $acceleration = new AccelerationConfig(
 *     provider: 'cuda',
 *     deviceId: 0,
 * );
 * ```
 */
readonly class AccelerationConfig
{
    /**
     * @param string $provider Execution provider: "auto" (default), "cpu", "coreml", "cuda", "tensorrt"
     * @param int $deviceId GPU device ID (for CUDA/TensorRT). Default is 0.
     */
    public function __construct(
        public string $provider = 'auto',
        public int $deviceId = 0,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $provider = $data['provider'] ?? 'auto';
        $deviceId = $data['device_id'] ?? 0;

        return new self(
            provider: is_string($provider) ? $provider : 'auto',
            deviceId: is_int($deviceId) ? $deviceId : 0,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        $result = [];

        // Only add non-default values
        if ($this->provider !== 'auto') {
            $result['provider'] = $this->provider;
        }
        if ($this->deviceId !== 0) {
            $result['device_id'] = $this->deviceId;
        }

        return $result;
    }
}
