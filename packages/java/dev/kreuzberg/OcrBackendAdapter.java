package dev.kreuzberg;

import java.util.List;
import java.util.Map;

/**
 * Path A Bridge implementation for IOcrBackend.
 *
 * Wraps a user-supplied implementation and delegates all method calls.
 * This adapter conforms to the hand-authored sealed interface.
 */
public final class OcrBackendAdapter implements IOcrBackend {
    private final IOcrBackend impl;

    public OcrBackendAdapter(IOcrBackend impl) {
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
    public ExtractionResult processImage(byte[] image_bytes, OcrConfig config) throws Exception {
        return impl.processImage(image_bytes, config);
    }

    @Override
    public ExtractionResult processImageFile(java.nio.file.Path path, OcrConfig config) throws Exception {
        return impl.processImageFile(path, config);
    }

    @Override
    public boolean supportsLanguage(String lang) throws Exception {
        return impl.supportsLanguage(lang);
    }

    @Override
    public String backendType() throws Exception {
        return impl.backendType();
    }

    @Override
    public List<String> supportedLanguages() throws Exception {
        return impl.supportedLanguages();
    }

    @Override
    public boolean supportsTableDetection() throws Exception {
        return impl.supportsTableDetection();
    }

    @Override
    public boolean supportsDocumentProcessing() throws Exception {
        return impl.supportsDocumentProcessing();
    }

    @Override
    public ExtractionResult processDocument(java.nio.file.Path _path, OcrConfig _config) throws Exception {
        return impl.processDocument(_path, _config);
    }


}
