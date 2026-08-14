#include "entitlement_runtime.h"

#include <Windows.h>

#include "validation.h"

namespace sunrise::state::entitlements {
namespace {

SRWLOCK g_lock{SRWLOCK_INIT};

/** @return Process-wide policy storage, seeded with the bundled policy on first use. */
[[nodiscard]] Table& storage() noexcept {
    static Table table = authored();
    return table;
}

} // namespace

/** Publishes the immutable ownership policy for this process. */
bool publish(const Table& table) noexcept {
    if (!valid(table)) {
        return false;
    }
    AcquireSRWLockExclusive(&g_lock);
    storage() = table;
    ReleaseSRWLockExclusive(&g_lock);
    return true;
}

/** @return The active ownership policy, or the bundled policy when none was published. */
const Table& get() noexcept {
    return storage();
}

/** Restores the bundled ownership policy. */
void clear() noexcept {
    AcquireSRWLockExclusive(&g_lock);
    storage() = authored();
    ReleaseSRWLockExclusive(&g_lock);
}

} // namespace sunrise::state::entitlements
