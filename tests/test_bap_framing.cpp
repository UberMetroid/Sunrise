#include <array>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <span>
#include <string_view>

#include "middleware/bap/frame.h"

extern void register_test(const char* name, bool (*func)());

namespace {

bool test_bap_frame_roundtrip() {
    using namespace sunrise::middleware::bap;

    std::array<std::byte, 128> buffer{};
    std::array<std::byte, 16> payload = {
        std::byte{0xDE}, std::byte{0xAD}, std::byte{0xBE}, std::byte{0xEF},
        std::byte{0x01}, std::byte{0x02}, std::byte{0x03}, std::byte{0x04}
    };

    std::size_t written = 0;
    bool encode_ok = encode_frame(FrameType::plaintext2, payload, buffer, written);
    if (!encode_ok || written != 6 + payload.size()) {
        return false;
    }

    OuterFrame frame{};
    bool parse_ok = parse_frame(std::span(buffer.data(), written), frame);
    if (!parse_ok) {
        return false;
    }

    if (frame.frameType != FrameType::plaintext2) {
        return false;
    }
    if (frame.payload.size() != payload.size()) {
        return false;
    }

    for (std::size_t i = 0; i < payload.size(); ++i) {
        if (frame.payload[i] != payload[i]) {
            return false;
        }
    }

    return true;
}

bool test_bap_response_payload() {
    using namespace sunrise::middleware::bap;

    std::array<std::byte, 128> buffer{};
    std::array<std::byte, 4> body = { std::byte{0x11}, std::byte{0x22}, std::byte{0x33}, std::byte{0x44} };

    std::size_t written = 0;
    bool ok = encode_response_payload(ResponseService::serverHello, 0x12345678, body, buffer, written);
    if (!ok || written != 8 + body.size()) {
        return false;
    }

    // Verify header fields: service=26 (0x001A), taskId=0x12345678, status=200 (0x00C8)
    if (buffer[0] != std::byte{0x00} || buffer[1] != std::byte{0x1A}) return false;
    if (buffer[2] != std::byte{0x12} || buffer[3] != std::byte{0x34} ||
        buffer[4] != std::byte{0x56} || buffer[5] != std::byte{0x78}) return false;
    if (buffer[6] != std::byte{0x00} || buffer[7] != std::byte{0xC8}) return false;

    return true;
}

struct RegisterBapTests {
    RegisterBapTests() {
        register_test("BAP Outer Frame Encode/Parse Roundtrip", test_bap_frame_roundtrip);
        register_test("BAP Response Payload Header & Body Encoding", test_bap_response_payload);
    }
} g_register;

} // namespace
