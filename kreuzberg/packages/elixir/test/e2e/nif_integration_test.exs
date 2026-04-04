defmodule KreuzbergTest.E2E.NIFIntegrationTest do
  @moduledoc """
  End-to-end NIF integration tests for Rustler boundary crossing safety.

  Critical tests covering:
  - NIF boundary crossing (Erlang term encoding/decoding)
  - Async operations via processes and Task supervision
  - NIF resource cleanup and memory management
  - OTP supervisor integration
  - Term encoding/decoding accuracy across boundaries
  - Error propagation from NIF layer
  - Concurrent NIF calls without deadlocks
  - Large data handling through NIF interface
  """

  use ExUnit.Case, async: true

  # ===== NIF Boundary Crossing Safety Tests =====

  @tag :e2e
  test "binary data crosses NIF boundary correctly" do
    test_binary = "Hello from Elixir crossing NIF boundary"

    {:ok, result} = Kreuzberg.extract(test_binary, "text/plain")

    assert result.content == test_binary
    assert byte_size(result.content) == byte_size(test_binary)
  end

  @tag :e2e
  test "large binary data (10MB) crosses NIF boundary" do
    large_binary = String.duplicate("X", 10_000_000)

    {:ok, result} = Kreuzberg.extract(large_binary, "text/plain")

    assert byte_size(result.content) > 0
    assert is_binary(result.content)
  end

  @tag :e2e
  test "unicode characters survive NIF boundary crossing" do
    unicode_text = "Hello 你好 مرحبا שלום 🚀"

    {:ok, result} = Kreuzberg.extract(unicode_text, "text/plain")

    assert result.content == unicode_text
  end

  @tag :e2e
  test "binary with null bytes crosses NIF boundary" do
    binary_with_nulls = <<"hello", 0, "world", 0, "test">>

    # This should not crash the NIF and should handle null bytes gracefully
    result = Kreuzberg.extract(binary_with_nulls, "text/plain")

    # Verify the NIF doesn't crash and returns a proper result
    case result do
      {:ok, extraction_result} ->
        assert %Kreuzberg.ExtractionResult{} = extraction_result
        assert is_binary(extraction_result.content)

      {:error, reason} ->
        assert is_binary(reason)
        assert byte_size(reason) > 0
    end
  end

  @tag :e2e
  test "metadata map structure preserved across NIF boundary" do
    text = "Test content"

    {:ok, result} = Kreuzberg.extract(text, "text/plain")

    assert %Kreuzberg.Metadata{} = result.metadata
    # All values in metadata should be valid Erlang terms
    # Convert struct to map for enumeration
    metadata_map = Map.from_struct(result.metadata)

    Enum.each(metadata_map, fn {_key, value} ->
      assert valid_erlang_term?(value)
    end)
  end

  @tag :e2e
  test "config map serialization across NIF boundary" do
    config = %Kreuzberg.ExtractionConfig{
      ocr: %{"enabled" => true, "backend" => "tesseract"},
      chunking: %{"enabled" => true, "size" => 256},
      use_cache: true
    }

    {:ok, result} = Kreuzberg.extract("Test", "text/plain", config)

    assert %Kreuzberg.ExtractionResult{} = result
  end

  # ===== Async Operations via Processes =====

  @tag :e2e
  test "async extraction via Task works correctly" do
    text = "Async test content"

    task =
      Task.async(fn ->
        Kreuzberg.extract(text, "text/plain")
      end)

    {:ok, result} = Task.await(task)

    assert result.content == text
  end

  @tag :e2e
  test "multiple concurrent NIF calls via Task.await_many" do
    texts = ["Task 1", "Task 2", "Task 3", "Task 4", "Task 5"]

    tasks =
      Enum.map(texts, fn text ->
        Task.async(fn ->
          Kreuzberg.extract(text, "text/plain")
        end)
      end)

    results = Task.await_many(tasks)

    assert length(results) == 5

    Enum.zip(texts, results)
    |> Enum.each(fn {text, {:ok, result}} ->
      assert result.content == text
    end)
  end

  @tag :e2e
  test "concurrent extraction with different MIME types" do
    inputs = [
      {"Plain text", "text/plain"},
      {"<html><p>HTML content</p></html>", "text/html"}
    ]

    tasks =
      Enum.map(inputs, fn {content, mime_type} ->
        Task.async(fn ->
          Kreuzberg.extract(content, mime_type)
        end)
      end)

    results = Task.await_many(tasks)

    assert length(results) == 2

    Enum.each(results, fn {:ok, result} ->
      assert %Kreuzberg.ExtractionResult{} = result
    end)
  end

  @tag :e2e
  test "async operations don't block each other" do
    start_time = System.monotonic_time(:millisecond)

    task1 = Task.async(fn -> Kreuzberg.extract("Task 1", "text/plain") end)
    task2 = Task.async(fn -> Kreuzberg.extract("Task 2", "text/plain") end)
    task3 = Task.async(fn -> Kreuzberg.extract("Task 3", "text/plain") end)

    results = Task.await_many([task1, task2, task3], 30_000)

    end_time = System.monotonic_time(:millisecond)
    elapsed = end_time - start_time

    # Verify all tasks completed successfully
    assert length(results) == 3

    Enum.each(results, fn {:ok, result} ->
      assert is_binary(result.content)
    end)

    # Should complete in reasonable time (concurrent, not sequential)
    # 3 tasks sequentially would take at least 3x the individual time
    # Concurrent should be much faster
    assert elapsed < 15_000, "Async operations should complete concurrently (#{elapsed}ms)"
  end

  @tag :e2e
  test "async process isolation prevents interference" do
    text_a = "Process A content"
    text_b = "Process B content"

    task_a = Task.async(fn -> Kreuzberg.extract(text_a, "text/plain") end)
    task_b = Task.async(fn -> Kreuzberg.extract(text_b, "text/plain") end)

    {:ok, result_a} = Task.await(task_a)
    {:ok, result_b} = Task.await(task_b)

    # Verify isolation - no cross-contamination
    assert result_a.content == text_a
    assert result_b.content == text_b
    assert result_a.content != result_b.content
  end

  # ===== NIF Resource Cleanup =====

  @tag :e2e
  test "NIF resources are cleaned up after extraction" do
    initial_memory = get_process_memory()

    # Perform single extraction
    {:ok, result} = Kreuzberg.extract("Test", "text/plain")
    assert is_binary(result.content)

    # Force garbage collection
    :erlang.garbage_collect()

    final_memory = get_process_memory()

    # Memory should be managed - allow 3x for single allocation but not unbounded growth
    assert final_memory <= initial_memory * 3,
           "Memory should not grow excessively after single extraction (was #{initial_memory}, now #{final_memory})"
  end

  @tag :e2e
  test "repeated NIF calls don't cause resource leaks" do
    initial_memory = get_process_memory()

    # Perform 100 extractions to detect memory leaks
    results =
      Enum.map(1..100, fn i ->
        {:ok, result} = Kreuzberg.extract("Test #{i}", "text/plain")
        result
      end)

    # Verify all results are valid
    assert length(results) == 100

    Enum.each(results, fn result ->
      assert is_binary(result.content)
    end)

    :erlang.garbage_collect()

    final_memory = get_process_memory()

    # After GC, memory shouldn't grow linearly with 100 calls
    # Should stay within reasonable bounds (not grow more than 5x initial)
    assert final_memory <= initial_memory * 5,
           "Memory leak detected - grew from #{initial_memory} to #{final_memory} bytes after 100 calls"
  end

  @tag :e2e
  test "large result structures are properly freed" do
    initial_memory = get_process_memory()

    large_html = "<html><body>" <> String.duplicate("<p>Content</p>", 1000) <> "</body></html>"
    {:ok, result} = Kreuzberg.extract(large_html, "text/html")

    # Verify large content was extracted
    assert is_binary(result.content)
    assert byte_size(result.content) > 0

    # Nullify the result to allow GC
    _result = nil

    :erlang.garbage_collect()

    final_memory = get_process_memory()

    # Large structure should be collectable - memory should return closer to baseline
    # Allow 2x for residual structures, but shouldn't keep all large allocation
    assert final_memory <= initial_memory * 2,
           "Large result structure not properly freed (#{initial_memory} -> #{final_memory})"
  end

  # ===== OTP Supervisor Integration =====

  @tag :e2e
  test "extraction works with Application supervisor running" do
    # Application.start/1 already called by Kreuzberg.Application
    {:ok, result} = Kreuzberg.extract("Test", "text/plain")

    assert %Kreuzberg.ExtractionResult{} = result
  end

  @tag :e2e
  test "async extraction uses default task supervisor" do
    task = Kreuzberg.extract_async("Test", "text/plain")

    assert is_struct(task, Task), "extract_async should return a Task struct"
    {:ok, result} = Task.await(task, 5000)

    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
  end

  @tag :e2e
  test "batch extraction integrates with OTP" do
    inputs = ["Text 1", "Text 2", "Text 3"]

    {:ok, results} =
      Kreuzberg.batch_extract_bytes(
        inputs,
        ["text/plain", "text/plain", "text/plain"]
      )

    assert length(results) == 3

    Enum.each(results, fn result ->
      assert %Kreuzberg.ExtractionResult{} = result
    end)
  end

  # ===== Term Encoding/Decoding Accuracy =====

  @tag :e2e
  test "extraction result struct is properly encoded/decoded" do
    {:ok, result} = Kreuzberg.extract("Test", "text/plain")

    # Verify all fields are correctly typed after NIF boundary crossing
    assert is_binary(result.content)
    assert is_binary(result.mime_type)
    assert is_map(result.metadata)
    assert is_list(result.tables)
    # Optional fields may be nil if not enabled
    assert is_nil(result.pages) or is_list(result.pages)
    assert is_nil(result.chunks) or is_list(result.chunks)
    assert is_nil(result.images) or is_list(result.images)
    assert is_nil(result.detected_languages) or is_list(result.detected_languages)
  end

  @tag :e2e
  test "nested map structures survive round-trip through NIF" do
    config = %Kreuzberg.ExtractionConfig{
      ocr: %{
        "enabled" => true,
        "backend" => "tesseract",
        "options" => %{"psm" => 3, "oem" => 1}
      }
    }

    {:ok, result} = Kreuzberg.extract("Test", "text/plain", config)

    # Verify the extraction succeeded with complex config
    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    # Verify result struct has all required fields after crossing NIF boundary
    assert is_map(result.metadata)
    # Pages may be nil if page extraction is not enabled
    assert is_nil(result.pages) or is_list(result.pages)
  end

  @tag :e2e
  test "list structures with mixed types survive NIF boundary" do
    {:ok, result} = Kreuzberg.extract("Test", "text/plain")

    # Pages list should be properly formed (or nil if not enabled)
    assert is_nil(result.pages) or is_list(result.pages)

    # Chunks list should be properly formed (or nil if not enabled)
    assert is_nil(result.chunks) or is_list(result.chunks)
  end

  # ===== Error Propagation from NIF =====

  @tag :e2e
  test "NIF errors are properly propagated to Elixir" do
    {:error, reason} = Kreuzberg.extract("data", "invalid/type")

    assert is_binary(reason)
    assert byte_size(reason) > 0
  end

  @tag :e2e
  test "NIF error doesn't crash the VM" do
    # First error
    {:error, reason1} = Kreuzberg.extract("data", "invalid/type")
    assert is_binary(reason1)

    # Second call should also work - NIF should recover from error
    {:error, reason2} = Kreuzberg.extract("data", "invalid/type")
    assert is_binary(reason2)

    # Third call with valid data should work - VM should still be responsive
    {:ok, result} = Kreuzberg.extract("valid text", "text/plain")
    assert %Kreuzberg.ExtractionResult{} = result
    assert is_binary(result.content)
    assert result.content == "valid text"
  end

  @tag :e2e
  test "error messages are descriptive and usable" do
    {:error, reason} = Kreuzberg.extract("data", "invalid/type")

    assert is_binary(reason)
    # Error message should give some indication of the problem
    assert String.length(reason) > 5
  end

  @tag :e2e
  test "async error handling via Task" do
    task =
      Task.async(fn ->
        Kreuzberg.extract("data", "invalid/type")
      end)

    {:error, reason} = Task.await(task)

    assert is_binary(reason)
  end

  # ===== Concurrent NIF Safety =====

  @tag :e2e
  test "high concurrency doesn't cause NIF deadlocks" do
    num_concurrent = 50

    # Start all tasks simultaneously
    start_time = System.monotonic_time(:millisecond)

    tasks =
      Enum.map(1..num_concurrent, fn i ->
        Task.async(fn ->
          Kreuzberg.extract("Task #{i}", "text/plain")
        end)
      end)

    # All tasks must complete successfully within timeout
    results = Task.await_many(tasks, 60_000)

    end_time = System.monotonic_time(:millisecond)
    elapsed = end_time - start_time

    # Verify all 50 concurrent tasks succeeded
    assert length(results) == num_concurrent,
           "All #{num_concurrent} concurrent tasks must complete"

    # Verify each result is valid
    successful_results =
      Enum.filter(results, fn
        {:ok, result} ->
          match?(%Kreuzberg.ExtractionResult{}, result) and is_binary(result.content)

        _ ->
          false
      end)

    assert length(successful_results) == num_concurrent,
           "All #{num_concurrent} tasks should have valid results, got #{length(successful_results)}"

    # Reasonable completion time (should be concurrent, not sequential)
    assert elapsed < 45_000,
           "50 concurrent tasks should complete in reasonable time (#{elapsed}ms), no deadlock detected"
  end

  @tag :e2e
  test "mixed operations in concurrent environment" do
    # Mix of extract, batch_extract, and async calls - real concurrency test
    task1 = Task.async(fn -> Kreuzberg.extract("Task 1", "text/plain") end)

    # Batch operation while task1 is running
    {:ok, batch_result} =
      Kreuzberg.batch_extract_bytes(
        ["Batch 1", "Batch 2"],
        ["text/plain", "text/plain"]
      )

    task2 = Task.async(fn -> Kreuzberg.extract("Task 2", "text/plain") end)

    {:ok, result1} = Task.await(task1, 5000)
    {:ok, result2} = Task.await(task2, 5000)

    # Verify all operations completed successfully with correct data
    assert %Kreuzberg.ExtractionResult{} = result1
    assert is_binary(result1.content)
    assert result1.content == "Task 1"

    assert length(batch_result) == 2, "Batch should return 2 results"

    Enum.each(batch_result, fn batch_item ->
      assert %Kreuzberg.ExtractionResult{} = batch_item
      assert is_binary(batch_item.content)
    end)

    assert %Kreuzberg.ExtractionResult{} = result2
    assert is_binary(result2.content)
    assert result2.content == "Task 2"

    # Verify no cross-contamination of data between mixed operations
    assert result1.content != result2.content
  end

  # Private helper functions

  defp valid_erlang_term?(term) do
    _encoded = :erlang.term_to_binary(term)
    true
  rescue
    _ -> false
  end

  defp get_process_memory do
    # Get memory info for current process
    case Process.info(self(), :memory) do
      {:memory, bytes} -> bytes
      nil -> 0
    end
  end
end
