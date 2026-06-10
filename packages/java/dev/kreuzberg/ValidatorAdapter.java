package dev.kreuzberg;

import java.util.List;
import java.util.Map;

/**
 * Path A Bridge implementation for IValidator.
 *
 * Wraps a user-supplied implementation and delegates all method calls.
 * This adapter conforms to the hand-authored sealed interface.
 */
public final class ValidatorAdapter implements IValidator {
    private final IValidator impl;

    public ValidatorAdapter(IValidator impl) {
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
    public void validate(ExtractionResult result, ExtractionConfig config) throws Exception {
        impl.validate(result, config);
    }

    @Override
    public boolean shouldValidate(ExtractionResult _result, ExtractionConfig _config) throws Exception {
        return impl.shouldValidate(_result, _config);
    }

    @Override
    public int priority() throws Exception {
        return impl.priority();
    }


}
