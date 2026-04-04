package dev.kreuzberg.config;

import dev.kreuzberg.KreuzbergException;
import dev.kreuzberg.ValidationHelper;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

/**
 * OCR configuration options.
 *
 * <p>
 * Configures OCR backend and language settings for text extraction from images.
 *
 * @since 4.0.0
 */
public final class OcrConfig {
	private final String backend;
	private final String language;
	private final TesseractConfig tesseractConfig;
	private final OutputFormat outputFormat;
	private final PaddleOcrConfig paddleOcrConfig;
	private final OcrElementConfig elementConfig;

	private OcrConfig(Builder builder) {
		this.backend = builder.backend;
		this.language = builder.language;
		this.tesseractConfig = builder.tesseractConfig;
		this.outputFormat = builder.outputFormat;
		this.paddleOcrConfig = builder.paddleOcrConfig;
		this.elementConfig = builder.elementConfig;
	}

	/**
	 * Creates a new builder for OCR configuration.
	 *
	 * @return a new builder instance
	 */
	public static Builder builder() {
		return new Builder();
	}

	/**
	 * Gets the OCR backend name.
	 *
	 * @return the backend name (e.g., "tesseract")
	 */
	public String getBackend() {
		return backend;
	}

	/**
	 * Gets the OCR language code.
	 *
	 * @return the language code (e.g., "eng", "deu")
	 */
	public String getLanguage() {
		return language;
	}

	/**
	 * Gets the Tesseract-specific configuration string.
	 *
	 * @return the tesseract config, or null if not set
	 */
	public TesseractConfig getTesseractConfig() {
		return tesseractConfig;
	}

	/**
	 * Gets the output format for OCR results.
	 *
	 * @return the output format, or null if not set (defaults to plain text)
	 */
	public OutputFormat getOutputFormat() {
		return outputFormat;
	}

	/**
	 * Gets the PaddleOCR-specific configuration.
	 *
	 * @return the paddle ocr config, or null if not set
	 */
	public PaddleOcrConfig getPaddleOcrConfig() {
		return paddleOcrConfig;
	}

	/**
	 * Gets the OCR element extraction configuration.
	 *
	 * @return the element config, or null if not set
	 */
	public OcrElementConfig getElementConfig() {
		return elementConfig;
	}

	/**
	 * Converts this configuration to a map for FFI.
	 *
	 * @return a map representation
	 */
	public Map<String, Object> toMap() {
		Map<String, Object> map = new HashMap<>();
		if (backend != null) {
			map.put("backend", backend);
		}
		if (language != null) {
			map.put("language", language);
		}
		if (tesseractConfig != null) {
			map.put("tesseract_config", tesseractConfig.toMap());
		}
		if (outputFormat != null) {
			map.put("output_format", outputFormat.getValue());
		}
		if (paddleOcrConfig != null) {
			map.put("paddle_ocr_config", paddleOcrConfig.toMap());
		}
		if (elementConfig != null) {
			map.put("element_config", elementConfig.toMap());
		}
		return map;
	}

	static OcrConfig fromMap(Map<String, Object> map) {
		if (map == null) {
			return null;
		}
		Builder builder = builder();
		Object backendValue = map.get("backend");
		if (backendValue instanceof String) {
			builder.backend((String) backendValue);
		}
		Object languageValue = map.get("language");
		if (languageValue instanceof String) {
			builder.language((String) languageValue);
		}
		Map<String, Object> tesseractMap = toMap(map.get("tesseract_config"));
		if (tesseractMap != null) {
			builder.tesseractConfig(TesseractConfig.fromMap(tesseractMap));
		}
		Object outputFormatValue = map.get("output_format");
		if (outputFormatValue instanceof String) {
			builder.outputFormat(OutputFormat.fromValue((String) outputFormatValue));
		}
		Map<String, Object> paddleMap = toMap(map.get("paddle_ocr_config"));
		if (paddleMap != null) {
			builder.paddleOcrConfig(PaddleOcrConfig.fromMap(paddleMap));
		}
		Map<String, Object> elementMap = toMap(map.get("element_config"));
		if (elementMap != null) {
			builder.elementConfig(OcrElementConfig.fromMap(elementMap));
		}
		return builder.build();
	}

	/** Builder for {@link OcrConfig}. */
	public static final class Builder {
		private String backend = "tesseract";
		private String language = "eng";
		private TesseractConfig tesseractConfig;
		private OutputFormat outputFormat;
		private PaddleOcrConfig paddleOcrConfig;
		private OcrElementConfig elementConfig;

		private Builder() {
		}

		/**
		 * Sets the OCR backend.
		 *
		 * @param backend
		 *            the backend name
		 * @return this builder
		 */
		public Builder backend(String backend) {
			try {
				ValidationHelper.validateOcrBackend(backend);
			} catch (KreuzbergException e) {
				throw new IllegalArgumentException(e.getMessage(), e);
			}
			this.backend = backend;
			return this;
		}

		/**
		 * Sets the OCR language.
		 *
		 * @param language
		 *            the language code
		 * @return this builder
		 */
		public Builder language(String language) {
			try {
				ValidationHelper.validateLanguageCode(language);
			} catch (KreuzbergException e) {
				throw new IllegalArgumentException(e.getMessage(), e);
			}
			this.language = language;
			return this;
		}

		/**
		 * Sets Tesseract-specific configuration.
		 *
		 * @param tesseractConfig
		 *            the tesseract config string
		 * @return this builder
		 */
		public Builder tesseractConfig(TesseractConfig tesseractConfig) {
			this.tesseractConfig = tesseractConfig;
			return this;
		}

		/**
		 * Sets the output format for OCR results.
		 *
		 * @param outputFormat
		 *            the output format (plain, markdown, djot, html)
		 * @return this builder
		 */
		public Builder outputFormat(OutputFormat outputFormat) {
			this.outputFormat = outputFormat;
			return this;
		}

		/**
		 * Sets the PaddleOCR-specific configuration.
		 *
		 * @param paddleOcrConfig
		 *            the paddle ocr config
		 * @return this builder
		 */
		public Builder paddleOcrConfig(PaddleOcrConfig paddleOcrConfig) {
			this.paddleOcrConfig = paddleOcrConfig;
			return this;
		}

		/**
		 * Sets the OCR element extraction configuration.
		 *
		 * @param elementConfig
		 *            the element config
		 * @return this builder
		 */
		public Builder elementConfig(OcrElementConfig elementConfig) {
			this.elementConfig = elementConfig;
			return this;
		}

		/**
		 * Builds the OCR configuration.
		 *
		 * @return the built configuration
		 */
		public OcrConfig build() {
			return new OcrConfig(this);
		}
	}

	@SuppressWarnings("unchecked")
	private static Map<String, Object> toMap(Object value) {
		if (value instanceof Map) {
			return (Map<String, Object>) value;
		}
		return Collections.emptyMap();
	}
}
