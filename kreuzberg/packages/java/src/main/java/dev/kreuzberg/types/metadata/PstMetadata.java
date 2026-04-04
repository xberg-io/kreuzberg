package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;

/** Outlook PST archive metadata. */
public record PstMetadata(@JsonProperty("message_count") int messageCount) {
}
