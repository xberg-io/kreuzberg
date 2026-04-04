# frozen_string_literal: true

require 'mkmf'
require 'rb_sys/mkmf'
require 'rbconfig'
require 'fileutils'

if Gem.win_platform?
  # Use CI-provided CARGO_TARGET_DIR if available, otherwise use a short path
  # GitHub Actions sets CARGO_TARGET_DIR=C:\t for MAX_PATH mitigation
  if ENV['CARGO_TARGET_DIR']
    puts "Windows detected: Using existing CARGO_TARGET_DIR=#{ENV['CARGO_TARGET_DIR']}"
  else
    short_target_dir = Dir.exist?('C:/t') ? 'C:/t' : 'C:/kz-build'
    begin
      FileUtils.mkdir_p(short_target_dir)
      ENV['CARGO_TARGET_DIR'] = short_target_dir
      ENV['OUT_DIR'] = short_target_dir
      puts "Windows detected: Using short build path #{short_target_dir}"
    rescue StandardError => e
      puts "Warning: Could not create short path #{short_target_dir}: #{e.message}"
    end
  end
end

if /mswin|mingw/.match?(RbConfig::CONFIG['host_os'])
  devkit = ENV.fetch('RI_DEVKIT', nil)
  prefix = ENV['MSYSTEM_PREFIX'] || '/ucrt64'

  # Set up include paths for MSVC compatibility headers
  native_include = File.expand_path('ext/kreuzberg_rb/native/include', __dir__).tr('\\', '/')
  compat_include = File.expand_path('ext/kreuzberg_rb/native/include/msvc_compat', __dir__).tr('\\', '/')

  extra_args = []
  extra_args << "-I#{native_include}"
  extra_args << "-I#{compat_include}"
  extra_args << '-fms-extensions'
  extra_args << '-fno-omit-frame-pointer'

  if devkit
    sysroot = "#{devkit}#{prefix}".tr('\\', '/')
    extra_args.push('--target=x86_64-pc-windows-gnu', "--sysroot=#{sysroot}")
  end

  unless extra_args.empty?
    existing = ENV['BINDGEN_EXTRA_CLANG_ARGS'].to_s.split(/\s+/).reject(&:empty?)
    ENV['BINDGEN_EXTRA_CLANG_ARGS'] = (existing + extra_args).uniq.join(' ')
    puts "BINDGEN_EXTRA_CLANG_ARGS set to: #{ENV.fetch('BINDGEN_EXTRA_CLANG_ARGS', nil)}"
  end

  # Set target for Windows GNU toolchain if not already set
  ENV['CARGO_BUILD_TARGET'] ||= 'x86_64-pc-windows-gnu' if devkit || ENV['MSYSTEM']
end

default_profile = ENV.fetch('CARGO_PROFILE', 'release')

create_rust_makefile('kreuzberg_rb') do |config|
  config.profile = default_profile.to_sym
  config.ext_dir = File.expand_path('ext/kreuzberg_rb/native', __dir__)
end
