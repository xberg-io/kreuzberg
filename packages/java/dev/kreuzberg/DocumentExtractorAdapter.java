package dev.kreuzberg;

import java.util.List;
import java.util.Map;

/**
 * Path A Bridge implementation for IDocumentExtractor.
 *
 * Wraps a user-supplied implementation and delegates all method calls.
 * This adapter conforms to the hand-authored sealed interface.
 */
public final class DocumentExtractorAdapter implements IDocumentExtractor {
    private final IDocumentExtractor impl;

    public DocumentExtractorAdapter(IDocumentExtractor impl) {
        this.impl = impl;
    }

    @Override
    public String name() {
        return impl.name();
    }

    @Override
    public String version() {
        return impl.version();
    }

    @Override
    public void initialize() throws Exception {
        impl.initialize();
    }

    @Override
    public void shutdown() throws Exception {
        impl.shutdown();
    }

    @Override
    public String extractBytes(byte[] content, String mime_type, ExtractionConfig config) throws Exception {
        return impl.extractBytes(content, mime_type, config);
    }

    @Override
    public String extractFile(java.nio.file.Path path, String mime_type, ExtractionConfig config) throws Exception {
        return impl.extractFile(path, mime_type, config);
    }

    @Override
    public List<String> supportedMimeTypes() throws Exception {
        return impl.supportedMimeTypes();
    }

    @Override
    public int priority() throws Exception {
        return impl.priority();
    }

    @Override
    public boolean canHandle(java.nio.file.Path _path, String _mime_type) throws Exception {
        return impl.canHandle(_path, _mime_type);
    }


}
