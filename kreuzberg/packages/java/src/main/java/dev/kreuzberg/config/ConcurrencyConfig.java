package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Thread and concurrency limits for constrained environments.
 *
 * @since 4.7.0
 */
public final class ConcurrencyConfig {
	private final Integer maxThreads;

	private ConcurrencyConfig(Builder builder) {
		this.maxThreads = builder.maxThreads;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Get the maximum number of threads for all internal thread pools.
	 *
	 * @return the maximum thread count, or null if not set
	 */
	public Integer getMaxThreads() {
		return maxThreads;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (maxThreads != null) {
			map.put("max_threads", maxThreads);
		}
		return map;
	}

	public static final class Builder {
		private Integer maxThreads;

		private Builder() {
		}

		/**
		 * Set the maximum number of threads for all internal thread pools.
		 *
		 * @param maxThreads
		 *            the maximum thread count
		 * @return this builder for chaining
		 */
		public Builder maxThreads(Integer maxThreads) {
			this.maxThreads = maxThreads;
			return this;
		}

		public ConcurrencyConfig build() {
			return new ConcurrencyConfig(this);
		}
	}

	static ConcurrencyConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object maxThreadsValue = map.get("max_threads");
		if (maxThreadsValue instanceof Number) {
			builder.maxThreads(((Number) maxThreadsValue).intValue());
		}
		return builder.build();
	}
}
