package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Confidence scores for OCR detection and recognition.
 *
 * <p>
 * Represents the confidence scores from the OCR backend for element detection
 * and text recognition.
 *
 * @since 4.4.0
 */
public final class OcrConfidence {
	private final Double detection;
	private final Double recognition;

	@JsonCreator
	public OcrConfidence(@JsonProperty("detection") Double detection, @JsonProperty("recognition") Double recognition) {
		this.detection = detection;
		this.recognition = recognition;
	}

	@JsonProperty("detection")
	public Double getDetection() {
		return detection;
	}

	@JsonProperty("recognition")
	public Double getRecognition() {
		return recognition;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof OcrConfidence)) {
			return false;
		}
		OcrConfidence other = (OcrConfidence) obj;
		return Objects.equals(detection, other.detection) && Objects.equals(recognition, other.recognition);
	}

	@Override
	public int hashCode() {
		return Objects.hash(detection, recognition);
	}

	@Override
	public String toString() {
		return "OcrConfidence{" + "detection=" + detection + ", recognition=" + recognition + '}';
	}
}
