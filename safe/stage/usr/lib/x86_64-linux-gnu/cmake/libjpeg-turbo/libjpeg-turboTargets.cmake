get_filename_component(_libjpeg_turbo_prefix "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)

if(NOT TARGET jpeg)
  add_library(jpeg SHARED IMPORTED)
  set_target_properties(jpeg PROPERTIES
    IMPORTED_LOCATION "${_libjpeg_turbo_prefix}/lib/x86_64-linux-gnu/libjpeg.so"
    IMPORTED_SONAME "libjpeg.so.8"
    INTERFACE_INCLUDE_DIRECTORIES "${_libjpeg_turbo_prefix}/include;${_libjpeg_turbo_prefix}/include/x86_64-linux-gnu"
  )
endif()

if(NOT TARGET turbojpeg)
  add_library(turbojpeg SHARED IMPORTED)
  set_target_properties(turbojpeg PROPERTIES
    IMPORTED_LOCATION "${_libjpeg_turbo_prefix}/lib/x86_64-linux-gnu/libturbojpeg.so"
    IMPORTED_SONAME "libturbojpeg.so.0"
    INTERFACE_LINK_LIBRARIES "jpeg"
    INTERFACE_INCLUDE_DIRECTORIES "${_libjpeg_turbo_prefix}/include;${_libjpeg_turbo_prefix}/include/x86_64-linux-gnu"
  )
endif()
