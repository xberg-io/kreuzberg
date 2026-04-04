# frozen_string_literal: true

module Kreuzberg
  # @example Extract a file
  # @example Detect file type
  module CLI
    module_function

    # Extract content from a file using the CLI
    #
    # @param path_or_nil [String, nil] Path to the file (positional, for backward compatibility)
    # @param path [String] Path to the file (keyword argument)
    # @param output [String] Output format ("text", "json", "markdown")
    # @param ocr [Boolean] Enable OCR
    # @return [String] Extracted content
    #
    def extract(path_or_nil = nil, path: nil, output: 'text', ocr: false)
      # Support both positional and keyword argument for path (backward compatibility)
      actual_path = path_or_nil || path
      raise ArgumentError, 'path is required' if actual_path.nil?

      args = ['extract', actual_path, '--format', output]
      args.push('--ocr', ocr ? 'true' : 'false')
      CLIProxy.call(args)
    end

    # Detect MIME type of a file using the CLI
    #
    # @param path_or_nil [String, nil] Path to the file (positional, for backward compatibility)
    # @param path [String] Path to the file (keyword argument)
    # @return [String] MIME type
    #
    def detect(path_or_nil = nil, path: nil)
      # Support both positional and keyword argument for path (backward compatibility)
      actual_path = path_or_nil || path
      raise ArgumentError, 'path is required' if actual_path.nil?

      CLIProxy.call(['detect', actual_path]).strip
    end

    # Get CLI version
    #
    # @return [String] Version string
    #
    def version
      CLIProxy.call(['--version']).strip
    end

    # Get CLI help text
    #
    # @return [String] Help text
    #
    def help
      CLIProxy.call(['--help'])
    end
  end
end
