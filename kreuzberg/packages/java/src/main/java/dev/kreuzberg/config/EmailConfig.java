package dev.kreuzberg.config;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration for email extraction settings.
 *
 * @since 4.6.0
 */
public final class EmailConfig {
	private final Integer msgFallbackCodepage;

	private EmailConfig(Builder builder) {
		this.msgFallbackCodepage = builder.msgFallbackCodepage;
	}

	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Get the fallback code page for MSG email body decoding.
	 *
	 * @return the fallback code page, or null if not set
	 */
	public Integer getMsgFallbackCodepage() {
		return msgFallbackCodepage;
	}

	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (msgFallbackCodepage != null) {
			map.put("msg_fallback_codepage", msgFallbackCodepage);
		}
		return map;
	}

	public static final class Builder {
		private Integer msgFallbackCodepage;

		private Builder() {
		}

		/**
		 * Set the fallback code page for MSG email body decoding.
		 *
		 * @param msgFallbackCodepage
		 *            the code page identifier
		 * @return this builder for chaining
		 */
		public Builder msgFallbackCodepage(Integer msgFallbackCodepage) {
			this.msgFallbackCodepage = msgFallbackCodepage;
			return this;
		}

		public EmailConfig build() {
			return new EmailConfig(this);
		}
	}

	static EmailConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object codepageValue = map.get("msg_fallback_codepage");
		if (codepageValue instanceof Number) {
			builder.msgFallbackCodepage(((Number) codepageValue).intValue());
		}
		return builder.build();
	}
}
