package dev.kreuzberg;

/**
 * Exception thrown when cache operations fail.
 *
 * <p>
 * Cache errors are typically non-fatal and occur during caching operations,
 * such as:
 *
 * <ul>
 * <li>Cache directory creation failures
 * <li>Disk write errors
 * <li>Cache entry corruption
 * <li>Insufficient disk space
 * </ul>
 *
 * <p>
 * These errors are usually logged but don't prevent extraction from completing.
 *
 * @since 4.0.0
 */
public final class CacheException extends KreuzbergException {
	/**
	 * Constructs a new cache exception with the specified message.
	 *
	 * @param message
	 *            the detail message explaining why the cache operation failed
	 */
	public CacheException(String message) {
		super(message);
	}

	/**
	 * Constructs a new cache exception with the specified message and cause.
	 *
	 * @param message
	 *            the detail message
	 * @param cause
	 *            the cause of the cache failure
	 */
	public CacheException(String message, Throwable cause) {
		super(message, cause);
	}
}
