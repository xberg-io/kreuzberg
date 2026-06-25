package dev.xberg;

/**
 * Path A Bridge implementation for IPostProcessor.
 *
 * Wraps a user-supplied implementation and delegates all method calls.
 * This adapter conforms to the hand-authored sealed interface.
 */
public final class PostProcessorAdapter implements IPostProcessor {
    private final IPostProcessor impl;

    public PostProcessorAdapter(IPostProcessor impl) {
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
    public void process(ExtractionResult result, ExtractionConfig config) throws Exception {
        impl.process(result, config);
    }

    @Override
    public String processing_stage() throws Exception {
        return impl.processing_stage();
    }

    @Override
    public boolean should_process(ExtractionResult _result, ExtractionConfig _config) throws Exception {
        return impl.should_process(_result, _config);
    }

    @Override
    public long estimated_duration_ms(ExtractionResult _result) throws Exception {
        return impl.estimated_duration_ms(_result);
    }

    @Override
    public int priority() throws Exception {
        return impl.priority();
    }


}
