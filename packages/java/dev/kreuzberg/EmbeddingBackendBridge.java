package dev.kreuzberg;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Allocates Panama FFM upcall stubs for an IEmbeddingBackend implementation,
 * assembles the C vtable in native memory, and provides static
 * registerEmbeddingBackend/unregisterEmbeddingBackend helpers.
 */
@SuppressWarnings("PMD")
public final class EmbeddingBackendBridge implements AutoCloseable {

    private static final Linker LINKER = Linker.nativeLinker();
    private static final MethodHandles.Lookup LOOKUP = MethodHandles.lookup();
    private static final ObjectMapper JSON = new ObjectMapper();

    /** Live registry — keeps Arenas and upcall stubs alive past the register call. */
    private static final ConcurrentHashMap<String, EmbeddingBackendBridge>
            EMBEDDING_BACKEND_BRIDGES = new ConcurrentHashMap<>();

    // C vtable: 8 fields (4 plugin methods + 2 trait methods + free_string + free_user_data)
    private static final MemoryLayout VTABLE_LAYOUT = MemoryLayout.structLayout(
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS,
            ValueLayout.ADDRESS
    );
    private static final long VTABLE_SIZE = VTABLE_LAYOUT.byteSize();

    private final Arena arena;
    private final MemorySegment vtable;
    private final IEmbeddingBackend impl;

    EmbeddingBackendBridge(final IEmbeddingBackend impl) {
        this.impl = impl;
        this.arena = Arena.ofShared();
        this.vtable = arena.allocate(VTABLE_SIZE);
        try {
            initializeStubs();
        } catch (ReflectiveOperationException e) {
            arena.close();
            throw new RuntimeException("Failed to create trait bridge stubs", e);
        }
    }

    private void initializeStubs() throws ReflectiveOperationException {
        long offset = 0L;

        var stubName = LINKER.upcallStub(LOOKUP.bind(this, "handleName",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubName);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubVersion = LINKER.upcallStub(LOOKUP.bind(this, "handleVersion",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubVersion);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubInitialize = LINKER.upcallStub(LOOKUP.bind(this, "handleInitialize",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubInitialize);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubShutdown = LINKER.upcallStub(LOOKUP.bind(this, "handleShutdown",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubShutdown);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubDimensions = LINKER.upcallStub(LOOKUP.bind(this, "handleDimensions",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubDimensions);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubEmbed = LINKER.upcallStub(LOOKUP.bind(this, "handleEmbed",
            MethodType.methodType(int.class, MemorySegment.class, MemorySegment.class, MemorySegment.class, MemorySegment.class)),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubEmbed);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubFreeString = LINKER.upcallStub(LOOKUP.bind(this, "freeString",
            MethodType.methodType(void.class, MemorySegment.class)),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubFreeString);
        offset += ValueLayout.ADDRESS.byteSize();

        var stubFreeUserData = LINKER.upcallStub(LOOKUP.bind(this, "freeUserData",
            MethodType.methodType(void.class, MemorySegment.class)),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS),
            arena);
        vtable.set(ValueLayout.ADDRESS, offset, stubFreeUserData);
        offset += ValueLayout.ADDRESS.byteSize();

    }

    MemorySegment vtableSegment() { return vtable; }

    private int handleName(MemorySegment userData, MemorySegment outName, MemorySegment outError) {
        try {
            outName.set(ValueLayout.ADDRESS, 0, arena.allocateFrom(impl.name()));
            return 0;
        } catch (Throwable e) { return 1; }
    }

    private int handleVersion(MemorySegment userData, MemorySegment outVersion, MemorySegment outError) {
        try {
            outVersion.set(ValueLayout.ADDRESS, 0, arena.allocateFrom(impl.version()));
            return 0;
        } catch (Throwable e) { return 1; }
    }

    private int handleInitialize(MemorySegment userData, MemorySegment outError) {
        try {
            impl.initialize();
            return 0;
        } catch (Throwable e) { return 1; }
    }

    private int handleShutdown(MemorySegment userData, MemorySegment outError) {
        try {
            impl.shutdown();
            return 0;
        } catch (Throwable e) { return 1; }
    }

    private int handleDimensions(MemorySegment userData, MemorySegment outResult, MemorySegment outError) {
        try {
            long result = impl.dimensions();
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private int handleEmbed(MemorySegment userData, MemorySegment texts_in, MemorySegment outResult, MemorySegment outError) {
        try {
            String texts_json = texts_in.reinterpret(Long.MAX_VALUE).getString(0);
            List<String> texts = JSON.readValue(texts_json, new com.fasterxml.jackson.core.type.TypeReference<List<String>>() { });
            List<List<Float>> result = impl.embed(texts);
            String json = JSON.writeValueAsString(result);
            MemorySegment jsonCs = arena.allocateFrom(json);
            outResult.set(ValueLayout.ADDRESS, 0, jsonCs);
            return 0;
        } catch (Throwable e) {
            writeError(outError, e);
            return 1;
        }
    }

    private void writeError(MemorySegment outError, Throwable e) {
        try { outError.set(ValueLayout.ADDRESS, 0, arena.allocateFrom(e.getClass().getSimpleName() + ": " + e.getMessage())); }
        catch (Throwable ignored) { /* swallow */ }
    }

    private void freeString(MemorySegment ptr) {
        // Strings returned by Java callbacks are arena-owned and released when this bridge closes.
    }

    private void freeUserData(MemorySegment userData) {
        // User data is Java-side state (the impl object), not freed by Rust on drop.
    }

    /** Read a NUL-terminated native C string safely without unbounded reinterpret. */
    private static String readNativeString(MemorySegment ptr) {
        return ptr.reinterpret(4096).getString(0);
    }

    @Override
    public void close() { arena.close(); }

    /** Register a EmbeddingBackend implementation via Panama FFM upcall stubs. */
    public static void registerEmbeddingBackend(final IEmbeddingBackend impl) throws Exception {
        var bridge = new EmbeddingBackendBridge(impl);
        try {
            try (var nameArena = Arena.ofShared()) {
                var nameCs = nameArena.allocateFrom(impl.name());
                MemorySegment outErr = nameArena.allocate(ValueLayout.ADDRESS);
                int rc = (int) NativeLib.KREUZBERG_REGISTER_EMBEDDING_BACKEND.invoke(
                    nameCs,
                    bridge.vtableSegment(),
                    MemorySegment.NULL,
                    outErr
                );
                if (rc != 0) {
                    MemorySegment errPtr = outErr.get(ValueLayout.ADDRESS, 0);
                    String msg = errPtr.equals(MemorySegment.NULL) ? "registration failed (rc=" + rc + ")" : readNativeString(errPtr);
                    throw new RuntimeException("registerEmbeddingBackend: " + msg);
                }
            }
        } catch (Throwable t) {
            bridge.close();
            if (t instanceof Exception e) {
                throw e;
            } else {
                throw new RuntimeException("Unexpected error during registration", t);
            }
        }
        EMBEDDING_BACKEND_BRIDGES.put(impl.name(), bridge);
    }

    /** Unregister a EmbeddingBackend implementation by name. */
    public static void unregisterEmbeddingBackend(String name) throws Exception {
        try {
            try (var nameArena = Arena.ofShared()) {
                var nameCs = nameArena.allocateFrom(name);
                MemorySegment outErr = nameArena.allocate(ValueLayout.ADDRESS);
                int rc = (int) NativeLib.KREUZBERG_UNREGISTER_EMBEDDING_BACKEND.invoke(nameCs, outErr);
                if (rc != 0) {
                    MemorySegment errPtr = outErr.get(ValueLayout.ADDRESS, 0);
                    String msg = errPtr.equals(MemorySegment.NULL)
                        ? "unregistration failed (rc=" + rc + ")"
                        : errPtr.reinterpret(Long.MAX_VALUE).getString(0);
                    throw new RuntimeException("unregisterEmbeddingBackend: " + msg);
                }
            }
        } catch (Throwable t) {
            if (t instanceof Exception e) {
                throw e;
            } else {
                throw new RuntimeException("Unexpected error during unregistration", t);
            }
        }
        EmbeddingBackendBridge old = EMBEDDING_BACKEND_BRIDGES.remove(name);
        if (old != null) { old.close(); }
    }
    /** Clear all registered EmbeddingBackend implementations. */
    public static void clearEmbeddingBackends() throws Exception {
        try {
            try (var arena = Arena.ofShared()) {
                MemorySegment outErr = arena.allocate(ValueLayout.ADDRESS);
                int rc = (int) NativeLib.KREUZBERG_CLEAR_EMBEDDING_BACKEND.invoke(outErr);
                if (rc != 0) {
                    MemorySegment errPtr = outErr.get(ValueLayout.ADDRESS, 0);
                    String msg = errPtr.equals(MemorySegment.NULL)
                        ? "clear failed (rc=" + rc + ")"
                        : errPtr.reinterpret(Long.MAX_VALUE).getString(0);
                    throw new RuntimeException("clearEmbeddingBackends: " + msg);
                }
            }
        } catch (Throwable t) {
            if (t instanceof Exception e) {
                throw e;
            } else {
                throw new RuntimeException("Unexpected error during clear", t);
            }
        }
        EMBEDDING_BACKEND_BRIDGES.values().forEach(EmbeddingBackendBridge::close);
        EMBEDDING_BACKEND_BRIDGES.clear();
    }
}
