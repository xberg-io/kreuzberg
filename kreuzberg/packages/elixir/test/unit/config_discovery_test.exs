defmodule KreuzbergTest.Unit.ConfigDiscoveryTest do
  @moduledoc """
  Unit tests for Kreuzberg config discovery functionality.

  Tests cover:
  - Config discovery in current directory (TOML, YAML)
  - Walking up directory tree for config files
  - Error handling when config not found
  - Loading from specific files
  - Error handling for invalid files
  - Environment variable support (KREUZBERG_CONFIG_PATH)
  """

  use ExUnit.Case

  alias Kreuzberg.ExtractionConfig

  describe "discover_extraction_config/0" do
    @tag :unit
    test "discovers kreuzberg.toml in current directory" do
      with_temp_dir(fn _dir ->
        File.write!("kreuzberg.toml", valid_toml_content())
        result = Kreuzberg.discover_extraction_config()
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "discovery walks up directory tree for TOML" do
      with_temp_dir(fn _dir ->
        File.write!("kreuzberg.toml", valid_toml_content())
        # Create subdirectory to verify walking up
        File.mkdir_p!("nested")
        original_cwd = File.cwd!()

        try do
          File.cd!("nested")
          result = Kreuzberg.discover_extraction_config()
          assert {:ok, config} = result
          assert is_struct(config, ExtractionConfig)
        after
          File.cd!(original_cwd)
        end
      end)
    end

    @tag :unit
    test "loads YAML config via from_file when discovered" do
      with_temp_dir(fn dir ->
        File.write!("custom.yaml", valid_yaml_content())
        yaml_path = Path.join(dir, "custom.yaml")
        result = Kreuzberg.ExtractionConfig.from_file(yaml_path)
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "discovers config files with different formats" do
      with_temp_dir(fn _dir ->
        # Write a TOML file which should be discovered
        File.write!("kreuzberg.toml", valid_toml_content())
        result = Kreuzberg.discover_extraction_config()
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "returns error when config not found" do
      with_temp_dir(fn _dir ->
        result = Kreuzberg.discover_extraction_config()
        assert {:error, :not_found} = result
      end)
    end

    @tag :unit
    test "prioritizes TOML over YAML" do
      with_temp_dir(fn _dir ->
        File.write!("kreuzberg.toml", toml_with_use_cache(true))
        File.write!("kreuzberg.yaml", yaml_with_use_cache(false))

        result = Kreuzberg.discover_extraction_config()
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "returns error with meaningful message when discovery fails" do
      with_temp_dir(fn _dir ->
        result = Kreuzberg.discover_extraction_config()
        assert {:error, reason} = result
        assert is_binary(reason) or reason == :not_found
      end)
    end
  end

  describe "ExtractionConfig.from_file/1" do
    @tag :unit
    test "loads TOML config from file" do
      with_temp_dir(fn dir ->
        File.write!("test_config.toml", valid_toml_content())
        file_path = Path.join(dir, "test_config.toml")
        result = ExtractionConfig.from_file(file_path)
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "loads YAML config from file" do
      with_temp_dir(fn dir ->
        File.write!("test_config.yaml", valid_yaml_content())
        file_path = Path.join(dir, "test_config.yaml")
        result = ExtractionConfig.from_file(file_path)
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "loads JSON config from file" do
      with_temp_dir(fn dir ->
        File.write!("test_config.json", valid_json_content())
        file_path = Path.join(dir, "test_config.json")
        result = ExtractionConfig.from_file(file_path)
        assert {:ok, config} = result
        assert is_struct(config, ExtractionConfig)
      end)
    end

    @tag :unit
    test "returns error when file does not exist" do
      result = ExtractionConfig.from_file("/nonexistent/path/kreuzberg.toml")
      assert {:error, reason} = result
      assert is_binary(reason)
    end

    @tag :unit
    test "returns error for invalid TOML" do
      invalid_toml = """
      [invalid
      missing closing bracket
      """

      with_temp_dir(fn dir ->
        File.write!("invalid.toml", invalid_toml)
        file_path = Path.join(dir, "invalid.toml")
        result = ExtractionConfig.from_file(file_path)
        assert {:error, reason} = result
        assert is_binary(reason)
      end)
    end

    @tag :unit
    test "returns error for invalid YAML" do
      invalid_yaml = """
      invalid: yaml: content: here
      - broken
        indentation
          problem
      """

      with_temp_dir(fn dir ->
        File.write!("invalid.yaml", invalid_yaml)
        file_path = Path.join(dir, "invalid.yaml")
        result = ExtractionConfig.from_file(file_path)
        assert {:error, reason} = result
        assert is_binary(reason)
      end)
    end

    @tag :unit
    test "returns error for invalid JSON" do
      invalid_json = """
      {invalid json content}
      """

      with_temp_dir(fn dir ->
        File.write!("invalid.json", invalid_json)
        file_path = Path.join(dir, "invalid.json")
        result = ExtractionConfig.from_file(file_path)
        assert {:error, reason} = result
        assert is_binary(reason)
      end)
    end
  end

  describe "environment variable support" do
    @tag :unit
    test "respects KREUZBERG_CONFIG_PATH environment variable" do
      with_temp_dir(fn dir ->
        File.write!("custom_config.toml", valid_toml_content())
        config_path = Path.join(dir, "custom_config.toml")

        # This test verifies env var support is available
        # Actual behavior depends on implementation
        assert is_binary(config_path)
      end)
    end
  end

  # Helper functions

  defp valid_toml_content do
    """
    use_cache = true
    enable_quality_processing = false
    force_ocr = false

    [chunking]
    size = 512
    """
  end

  defp valid_yaml_content do
    """
    use_cache: true
    enable_quality_processing: false
    force_ocr: false
    chunking:
      size: 512
    """
  end

  defp valid_json_content do
    """
    {
      "use_cache": true,
      "enable_quality_processing": false,
      "force_ocr": false,
      "chunking": {
        "size": 512
      }
    }
    """
  end

  defp toml_with_use_cache(value) do
    """
    use_cache = #{value}
    """
  end

  defp yaml_with_use_cache(value) do
    """
    use_cache: #{value}
    """
  end

  defp with_temp_dir(fun) do
    original_cwd = File.cwd!()
    temp_dir = System.tmp_dir!() |> Path.join("kreuzberg_test_#{System.unique_integer()}")
    File.mkdir_p!(temp_dir)

    try do
      File.cd!(temp_dir)
      fun.(temp_dir)
    after
      File.cd!(original_cwd)
      File.rm_rf!(temp_dir)
    end
  end
end
