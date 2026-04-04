package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Rotation information for an OCR element.
 *
 * <p>
 * Represents the detected rotation of an OCR element, including the angle in
 * degrees and associated confidence score.
 *
 * @since 4.4.0
 */
public final class OcrRotation {
	private final Double angleDegrees;
	private final Double confidence;

	@JsonCreator
	public OcrRotation(@JsonProperty("angle_degrees") Double angleDegrees,
			@JsonProperty("confidence") Double confidence) {
		this.angleDegrees = angleDegrees;
		this.confidence = confidence;
	}

	@JsonProperty("angle_degrees")
	public Double getAngleDegrees() {
		return angleDegrees;
	}

	@JsonProperty("confidence")
	public Double getConfidence() {
		return confidence;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof OcrRotation)) {
			return false;
		}
		OcrRotation other = (OcrRotation) obj;
		return Objects.equals(angleDegrees, other.angleDegrees) && Objects.equals(confidence, other.confidence);
	}

	@Override
	public int hashCode() {
		return Objects.hash(angleDegrees, confidence);
	}

	@Override
	public String toString() {
		return "OcrRotation{" + "angleDegrees=" + angleDegrees + ", confidence=" + confidence + '}';
	}
}
