get_filename_component(_libjpeg_turbo_prefix "${CMAKE_CURRENT_LIST_DIR}/../../../.." ABSOLUTE)
set(libjpeg_turbo_VERSION "2.1.5")
set(libjpeg_turbo_INCLUDE_DIRS
  "${_libjpeg_turbo_prefix}/include"
  "${_libjpeg_turbo_prefix}/include/x86_64-linux-gnu"
)
set(libjpeg_turbo_LIBRARY_DIR "${_libjpeg_turbo_prefix}/lib/x86_64-linux-gnu")
set(libjpeg_turbo_LIBRARIES jpeg turbojpeg)
include("${CMAKE_CURRENT_LIST_DIR}/libjpeg-turboTargets.cmake")
