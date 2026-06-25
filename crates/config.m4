dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl This file enables phpize to compile the extension using cargo instead of make.

PHP_ARG_ENABLE([xberg],
  [whether to enable the xberg extension],
  [AS_HELP_STRING([--enable-xberg],
    [Enable xberg extension support])],
  [yes])

if test "$PHP_XBERG_ENABLED" = "yes"; then
  dnl Check that cargo is available
  AC_PATH_PROG([CARGO], [cargo], [no])
  if test "x$CARGO" = "xno"; then
    AC_MSG_ERROR([cargo is required to build this extension])
  fi

  dnl Build the Rust extension using cargo
  AC_MSG_NOTICE([Building Rust extension xberg])

  dnl Set up the extension module
  PHP_NEW_EXTENSION(xberg, [], $ext_shared)

  dnl Custom build: invoke cargo instead of make
  PHP_ADD_BUILD_DIR($ext_builddir)

  dnl The actual build is handled by the build.rs script;
  dnl cargo outputs the .so/.dylib/.dll which phpize will place in extension_dir.
fi
