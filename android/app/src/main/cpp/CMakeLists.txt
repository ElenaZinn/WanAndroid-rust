cmake_minimum_required(VERSION 3.22.1)
project(wanandroid_core_jni)

add_library(wanandroid_core_jni SHARED jni_bridge.cpp)

add_library(wanandroid_core SHARED IMPORTED)
set_target_properties(wanandroid_core PROPERTIES
    IMPORTED_LOCATION "${CMAKE_CURRENT_LIST_DIR}/../../../../rust-lib/${ANDROID_ABI}/libwanandroid_core.so"
)

target_link_libraries(wanandroid_core_jni wanandroid_core log)
