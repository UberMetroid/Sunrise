#pragma once

#include <cstddef>
#include <cstdint>
#include <span>
#include <string_view>

namespace sunrise::middleware::signon {

/** Protobuf wire type for unsigned varint fields. */
inline constexpr unsigned kWireVarint = 0;
/** Protobuf wire type for length-delimited fields. */
inline constexpr unsigned kWireBytes = 2;
/** Protobuf field keys reserve three low bits for the wire type. */
inline constexpr unsigned kWireTypeBits = 3;
/** Low 7 bits of each protobuf varint byte carry value data. */
inline constexpr std::uint64_t kVarintPayloadMask = 0x7F;
/** Each protobuf varint byte advances the value by 7 bits. */
inline constexpr unsigned kVarintPayloadBits = 7;
/** High protobuf varint bit marks another byte as present. */
inline constexpr unsigned char kVarintContinuation = 0x80;

/** @param text Null-free ASCII field value. @return Byte view for a length-delimited field. */
[[nodiscard]] inline std::span<const std::byte> text_bytes(std::string_view text) noexcept {
    return std::as_bytes(std::span<const char>(text.data(), text.size()));
}

/** Fixed-storage protobuf writer shared by every SignOn sub-message. */
class Writer {
public:
    /** @param output Fixed caller-owned protobuf storage. */
    explicit Writer(std::span<std::byte> output) noexcept : output_(output) {}

    /**
     * Appends one protobuf varint field.
     * @param field Nonzero protobuf field number.
     * @param value Unsigned field value.
     * @return True when the key and value fit.
     */
    [[nodiscard]] bool varint(unsigned field, std::uint64_t value) noexcept {
        return raw_varint((static_cast<std::uint64_t>(field) << kWireTypeBits) | kWireVarint)
               && raw_varint(value);
    }

    /**
     * Appends one protobuf length-delimited field.
     * @param field Nonzero protobuf field number.
     * @param value Field payload bytes.
     * @return True when key, length, and payload fit.
     */
    [[nodiscard]] bool bytes(unsigned field, std::span<const std::byte> value) noexcept {
        const std::uint64_t key = (static_cast<std::uint64_t>(field) << kWireTypeBits) | kWireBytes;
        const std::size_t needed = varint_size(key) + varint_size(value.size()) + value.size();
        if (needed > output_.size() - size_) {
            return false;
        }
        (void)raw_varint(key);
        (void)raw_varint(value.size());
        for (const std::byte byte : value) {
            output_[size_++] = byte;
        }
        return true;
    }

    /** @return Bytes committed to the bound output. */
    [[nodiscard]] std::size_t size() const noexcept {
        return size_;
    }

private:
    /** @return Number of varint bytes needed to encode value. */
    [[nodiscard]] static constexpr std::size_t varint_size(std::uint64_t value) noexcept {
        std::size_t count = 0;
        do {
            ++count;
            value >>= kVarintPayloadBits;
        } while (value != 0);
        return count;
    }
    /**
     * Appends one unsigned protobuf varint.
     * @param value Value encoded in 7-bit groups.
     * @return True when the whole value fits.
     */
    [[nodiscard]] bool raw_varint(std::uint64_t value) noexcept {
        do {
            if (size_ == output_.size()) {
                return false;
            }
            unsigned char byte = static_cast<unsigned char>(value & kVarintPayloadMask);
            value >>= kVarintPayloadBits;
            if (value != 0) {
                byte |= kVarintContinuation;
            }
            output_[size_++] = static_cast<std::byte>(byte);
        } while (value != 0);
        return true;
    }

    std::span<std::byte> output_;
    std::size_t size_{};
};

} // namespace sunrise::middleware::signon
