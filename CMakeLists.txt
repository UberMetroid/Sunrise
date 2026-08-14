cmake_minimum_required(VERSION 3.20)
project(Sunrise VERSION 0.2.1 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Compiler flags
if (CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic -Wno-unused-parameter -Wno-missing-field-initializers)
endif()

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/Sunrise/src
    ${CMAKE_CURRENT_SOURCE_DIR}/Sunrise/vendor/imgui
    ${CMAKE_CURRENT_SOURCE_DIR}/Sunrise/vendor/detours
)

# Common state & middleware sources
file(GLOB_RECURSE MIDDLEWARE_SOURCES
    "Sunrise/src/middleware/bap/*.cpp"
    "Sunrise/src/middleware/compression/*.cpp"
    "Sunrise/src/middleware/content/*.cpp"
    "Sunrise/src/middleware/crypto/*.cpp"
    "Sunrise/src/middleware/datagen/*.cpp"
    "Sunrise/src/middleware/encoding/*.cpp"
    "Sunrise/src/middleware/protobuf/*.cpp"
    "Sunrise/src/middleware/queuez/*.cpp"
    "Sunrise/src/middleware/secure_channel/*.cpp"
    "Sunrise/src/middleware/signon/*.cpp"
    "Sunrise/src/middleware/web_service/*.cpp"
)

file(GLOB_RECURSE STATE_SOURCES
    "Sunrise/src/state/account/*.cpp"
    "Sunrise/src/state/activity/*.cpp"
    "Sunrise/src/state/build_data/*.cpp"
    "Sunrise/src/state/content/*.cpp"
    "Sunrise/src/state/content_manifest/*.cpp"
    "Sunrise/src/state/entitlements/*.cpp"
    "Sunrise/src/state/equipment/*.cpp"
    "Sunrise/src/state/matchmaking/*.cpp"
    "Sunrise/src/state/runtime/*.cpp"
    "Sunrise/src/state/unlocks/*.cpp"
)

file(GLOB_RECURSE CORE_PORTABLE_SOURCES
    "Sunrise/src/core/settings/*.cpp"
    "Sunrise/src/core/logging/snapshot/*.cpp"
    "Sunrise/src/core/logging/view/*.cpp"
)

# Target: Windows DLL (Cross-compile with MinGW or native MSVC)
if (WIN32 OR MINGW)
    add_definitions(-DUNICODE -D_UNICODE -DWIN32_LEAN_AND_MEAN)

    file(GLOB_RECURSE CLIENT_SOURCES "Sunrise/src/client/*.cpp")
    file(GLOB_RECURSE SERVER_SOURCES "Sunrise/src/server/*.cpp")
    file(GLOB_RECURSE STEAM_SOURCES "Sunrise/src/steam/*.cpp")
    file(GLOB_RECURSE CORE_SOURCES "Sunrise/src/core/*.cpp")
    file(GLOB VENDOR_SOURCES
        "Sunrise/vendor/detours/*.cpp"
        "Sunrise/vendor/imgui/*.cpp"
        "Sunrise/vendor/imgui/backends/*.cpp"
    )

    add_library(Sunrise SHARED
        Sunrise/src/dllmain.cpp
        ${CLIENT_SOURCES}
        ${CORE_SOURCES}
        ${MIDDLEWARE_SOURCES}
        ${SERVER_SOURCES}
        ${STATE_SOURCES}
        ${STEAM_SOURCES}
        ${VENDOR_SOURCES}
    )

    set_target_properties(Sunrise PROPERTIES
        OUTPUT_NAME "Sunrise"
        PREFIX ""
    )

    target_link_libraries(Sunrise PRIVATE
        ws2_32
        bcrypt
        d3d11
        dxgi
        user32
        gdi32
        ntdll
    )
else()
    # Native Linux Build: Test suite & Platform Shims
    add_definitions(-DSUNRISE_NATIVE_LINUX)
    include_directories(BEFORE ${CMAKE_CURRENT_SOURCE_DIR}/Sunrise/src/platform/shim)

    # Unit Tests Executable
    file(GLOB TEST_SOURCES "tests/*.cpp")

    add_executable(sunrise_tests
        ${TEST_SOURCES}
        ${MIDDLEWARE_SOURCES}
        ${STATE_SOURCES}
        ${CORE_PORTABLE_SOURCES}
    )

    target_link_libraries(sunrise_tests PRIVATE pthread crypto)

    enable_testing()
    add_test(NAME AllTests COMMAND sunrise_tests)
endif()
