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

get_filename_component(_FFI_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_FFI_PREFIX "${_FFI_CMAKE_DIR}/.." ABSOLUTE)

find_library(_FFI_LIBRARY
  NAMES kreuzberg_ffi libkreuzberg_ffi
  PATHS "${_FFI_PREFIX}/lib"
  NO_DEFAULT_PATH
)
if(NOT _FFI_LIBRARY)
  find_library(_FFI_LIBRARY NAMES kreuzberg_ffi libkreuzberg_ffi)
endif()

find_path(_FFI_INCLUDE_DIR
  NAMES kreuzberg.h
  PATHS "${_FFI_PREFIX}/include"
  NO_DEFAULT_PATH
)
if(NOT _FFI_INCLUDE_DIR)
  find_path(_FFI_INCLUDE_DIR NAMES kreuzberg.h)
endif()

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(kreuzberg-ffi
  REQUIRED_VARS _FFI_LIBRARY _FFI_INCLUDE_DIR
)

if(kreuzberg_ffi_FOUND)
  set(_FFI_LIB_TYPE UNKNOWN)
  if(_FFI_LIBRARY MATCHES "\\.(dylib|so)$" OR _FFI_LIBRARY MATCHES "\\.so\\.")
    set(_FFI_LIB_TYPE SHARED)
  elseif(_FFI_LIBRARY MATCHES "\\.dll$")
    set(_FFI_LIB_TYPE SHARED)
  elseif(_FFI_LIBRARY MATCHES "\\.(a|lib)$")
    set(_FFI_LIB_TYPE STATIC)
  endif()

  add_library(kreuzberg-ffi::kreuzberg-ffi ${_FFI_LIB_TYPE} IMPORTED)
  set_target_properties(kreuzberg-ffi::kreuzberg-ffi PROPERTIES
    IMPORTED_LOCATION "${_FFI_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${_FFI_INCLUDE_DIR}"
  )

  if(WIN32 AND _FFI_LIB_TYPE STREQUAL "SHARED")
    find_file(_FFI_DLL
      NAMES kreuzberg_ffi.dll libkreuzberg_ffi.dll
      PATHS "${_FFI_PREFIX}/bin" "${_FFI_PREFIX}/lib"
      NO_DEFAULT_PATH
    )
    if(_FFI_DLL)
      set_target_properties(kreuzberg-ffi::kreuzberg-ffi PROPERTIES
        IMPORTED_LOCATION "${_FFI_DLL}"
        IMPORTED_IMPLIB "${_FFI_LIBRARY}"
      )
    endif()
    unset(_FFI_DLL CACHE)
  endif()

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

  unset(_FFI_LIB_TYPE)
endif()

mark_as_advanced(_FFI_LIBRARY _FFI_INCLUDE_DIR)
unset(_FFI_CMAKE_DIR)
unset(_FFI_PREFIX)
