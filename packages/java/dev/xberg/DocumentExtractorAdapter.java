package dev.xberg;

import java.util.List;

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
    public String extract_bytes(byte[] content, String mime_type, ExtractionConfig config) throws Exception {
        return impl.extract_bytes(content, mime_type, config);
    }

    @Override
    public String extract_file(java.nio.file.Path path, String mime_type, ExtractionConfig config) throws Exception {
        return impl.extract_file(path, mime_type, config);
    }

    @Override
    public List<String> supported_mime_types() throws Exception {
        return impl.supported_mime_types();
    }

    @Override
    public int priority() throws Exception {
        return impl.priority();
    }

    @Override
    public boolean can_handle(java.nio.file.Path _path, String _mime_type) throws Exception {
        return impl.can_handle(_path, _mime_type);
    }


}
