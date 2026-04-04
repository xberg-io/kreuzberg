# kreuzberg-ffi CMake config-mode find module
#
# Defines the imported target:
#   kreuzberg-ffi::kreuzberg-ffi
#
# Usage:
#   find_package(kreuzberg-ffi REQUIRED)
#   target_link_libraries(myapp PRIVATE kreuzberg-ffi::kreuzberg-ffi)

if(TARGET kreuzberg-ffi::kreuzberg-ffi)
  return()
endif()

get_filename_component(_KREUZBERG_FFI_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_KREUZBERG_FFI_PREFIX "${_KREUZBERG_FFI_CMAKE_DIR}/.." ABSOLUTE)

# ── Step 1: Find the library and headers ──────────────────────────────

find_library(_KREUZBERG_FFI_LIBRARY
  NAMES kreuzberg_ffi libkreuzberg_ffi
  PATHS "${_KREUZBERG_FFI_PREFIX}/lib"
  NO_DEFAULT_PATH
)

if(NOT _KREUZBERG_FFI_LIBRARY)
  find_library(_KREUZBERG_FFI_LIBRARY
    NAMES kreuzberg_ffi libkreuzberg_ffi
  )
endif()

find_path(_KREUZBERG_FFI_INCLUDE_DIR
  NAMES kreuzberg.h
  PATHS "${_KREUZBERG_FFI_PREFIX}/include"
  NO_DEFAULT_PATH
)

if(NOT _KREUZBERG_FFI_INCLUDE_DIR)
  find_path(_KREUZBERG_FFI_INCLUDE_DIR NAMES kreuzberg.h)
endif()

# ── Step 2: Validate that required files were found ───────────────────

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(kreuzberg-ffi
  REQUIRED_VARS _KREUZBERG_FFI_LIBRARY _KREUZBERG_FFI_INCLUDE_DIR
)

# ── Step 3: Create the imported target with correct library type ──────

if(kreuzberg-ffi_FOUND)
  # Determine library type from the file extension
  set(_KREUZBERG_FFI_LIB_TYPE UNKNOWN)

  if(_KREUZBERG_FFI_LIBRARY MATCHES "\\.(dylib|so)$" OR _KREUZBERG_FFI_LIBRARY MATCHES "\\.so\\.")
    set(_KREUZBERG_FFI_LIB_TYPE SHARED)
  elseif(_KREUZBERG_FFI_LIBRARY MATCHES "\\.dll$")
    set(_KREUZBERG_FFI_LIB_TYPE SHARED)
  elseif(_KREUZBERG_FFI_LIBRARY MATCHES "\\.(a|lib)$")
    set(_KREUZBERG_FFI_LIB_TYPE STATIC)
  endif()

  add_library(kreuzberg-ffi::kreuzberg-ffi ${_KREUZBERG_FFI_LIB_TYPE} IMPORTED)

  # ── Step 4: Set target properties ─────────────────────────────────

  set_target_properties(kreuzberg-ffi::kreuzberg-ffi PROPERTIES
    IMPORTED_LOCATION "${_KREUZBERG_FFI_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${_KREUZBERG_FFI_INCLUDE_DIR}"
  )

  # On Windows with SHARED libraries, handle the DLL + import lib split
  if(WIN32 AND _KREUZBERG_FFI_LIB_TYPE STREQUAL "SHARED")
    # The found .dll.lib or .lib is the import library; find the actual DLL
    find_file(_KREUZBERG_FFI_DLL
      NAMES kreuzberg_ffi.dll libkreuzberg_ffi.dll
      PATHS "${_KREUZBERG_FFI_PREFIX}/bin" "${_KREUZBERG_FFI_PREFIX}/lib"
      NO_DEFAULT_PATH
    )
    if(_KREUZBERG_FFI_DLL)
      set_target_properties(kreuzberg-ffi::kreuzberg-ffi PROPERTIES
        IMPORTED_LOCATION "${_KREUZBERG_FFI_DLL}"
        IMPORTED_IMPLIB "${_KREUZBERG_FFI_LIBRARY}"
      )
    endif()
    unset(_KREUZBERG_FFI_DLL CACHE)
  endif()

  # Platform-specific link dependencies
  if(APPLE)
    set_property(TARGET kreuzberg-ffi::kreuzberg-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES "-framework CoreFoundation" "-framework Security" pthread)
  elseif(UNIX)
    set_property(TARGET kreuzberg-ffi::kreuzberg-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES pthread dl m)
  elseif(WIN32)
    set_property(TARGET kreuzberg-ffi::kreuzberg-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES ws2_32 userenv bcrypt)
  endif()

  unset(_KREUZBERG_FFI_LIB_TYPE)
endif()

mark_as_advanced(_KREUZBERG_FFI_LIBRARY _KREUZBERG_FFI_INCLUDE_DIR)
unset(_KREUZBERG_FFI_CMAKE_DIR)
unset(_KREUZBERG_FFI_PREFIX)
