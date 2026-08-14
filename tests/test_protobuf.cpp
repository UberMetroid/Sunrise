#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <span>
#include <string_view>

#include "middleware/protobuf/codec.h"

extern void register_test(const char* name, bool (*func)());

namespace {

bool test_protobuf_varint_and_bytes() {
    using namespace sunrise::middleware::protobuf;

    std::array<std::byte, 256> buffer{};
    Writer writer(buffer);

    // Field 1: Varint (12345)
    if (!writer.write_varint(1, 12345)) {
        return false;
    }

    // Field 2: Length delimited string/bytes
    const char* str = "Hello, Sunrise!";
    std::span<const std::byte> str_bytes = std::as_bytes(std::span(str, 15));
    if (!writer.write_length_delimited(2, str_bytes)) {
        return false;
    }

    // Read back fields
    Reader reader(std::span(buffer.data(), writer.size()));
    Field f1{};
    if (!reader.next(f1)) {
        return false;
    }
    if (f1.fieldNumber != 1 || f1.wireType != WireType::varint || f1.value != 12345) {
        return false;
    }

    Field f2{};
    if (!reader.next(f2)) {
        return false;
    }
    if (f2.fieldNumber != 2 || f2.wireType != WireType::lengthDelimited || f2.bytes.size() != 15) {
        return false;
    }

    if (std::memcmp(f2.bytes.data(), str, 15) != 0) {
        return false;
    }

    // No more fields
    Field f3{};
    if (reader.next(f3)) {
        return false;
    }

    return true;
}

bool test_signon_writer_bytes_safety() {
    using namespace sunrise::middleware::protobuf;

    // Test measuring fields
    std::size_t size = 0;
    if (!measure_varint_field(5, 999999, size)) {
        return false;
    }
    if (size == 0) {
        return false;
    }

    std::size_t ld_size = 0;
    if (!measure_length_delimited_field(10, 32, ld_size)) {
        return false;
    }
    if (ld_size < 34) {
        return false;
    }

    return true;
}

struct RegisterProtobufTests {
    RegisterProtobufTests() {
        register_test("Protobuf Writer/Reader Varint and LengthDelimited", test_protobuf_varint_and_bytes);
        register_test("Protobuf Measure Functions", test_signon_writer_bytes_safety);
    }
} g_register;

} // namespace
