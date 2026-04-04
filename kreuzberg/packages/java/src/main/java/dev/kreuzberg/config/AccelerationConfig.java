package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for ONNX Runtime execution provider selection.
 *
 * @since 4.5.0
 */
public final class AccelerationConfig {
	private final String provider;
	private final int deviceId;

	private AccelerationConfig(Builder builder) {
		this.provider = builder.provider;
		this.deviceId = builder.deviceId;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Get the execution provider name.
	 *
	 * @return the provider (auto, cpu, coreml, cuda, tensorrt)
	 */
	public String getProvider() {
		return provider;
	}

	/**
	 * Get the GPU device ID.
	 *
	 * @return the device ID (for CUDA/TensorRT)
	 */
	public int getDeviceId() {
		return deviceId;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (provider != null && !provider.isEmpty()) {
			map.put("provider", provider);
		}
		if (deviceId != 0) {
			map.put("device_id", deviceId);
		}
		return map;
	}

	public static final class Builder {
		private String provider = "auto";
		private int deviceId = 0;

		private Builder() {
		}

		/**
		 * Set the execution provider.
		 *
		 * @param provider
		 *            the provider name (auto, cpu, coreml, cuda, tensorrt)
		 * @return this builder for chaining
		 */
		public Builder provider(String provider) {
			this.provider = provider;
			return this;
		}

		/**
		 * Set the GPU device ID.
		 *
		 * @param deviceId
		 *            the device ID for CUDA/TensorRT
		 * @return this builder for chaining
		 */
		public Builder deviceId(int deviceId) {
			this.deviceId = deviceId;
			return this;
		}

		public AccelerationConfig build() {
			return new AccelerationConfig(this);
		}
	}

	static AccelerationConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		if (map.get("provider") instanceof String) {
			builder.provider((String) map.get("provider"));
		}
		Object deviceIdValue = map.get("device_id");
		if (deviceIdValue instanceof Number) {
			builder.deviceId(((Number) deviceIdValue).intValue());
		}
		return builder.build();
	}
}
