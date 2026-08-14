#include <string_view>

#include "core/settings/settings.h"

extern void register_test(const char* name, bool (*func)());

namespace {

bool test_settings_defaults() {
    using namespace sunrise::core::settings;

    Settings s = defaults();
    if (s.server.bapPort == 0) {
        return false;
    }
    if (s.client.userInterface.toggleVirtualKey == 0) {
        return false;
    }
    return true;
}

bool test_settings_json_parse_override() {
    using namespace sunrise::core::settings;

    const std::string_view json = R"json({
        "version": 3,
        "server": {
            "bapPort": 4321
        },
        "steam": {
            "user": {
                "personaName": "TestGuardian"
            }
        }
    })json";

    Settings output = defaults();
    bool ok = parse(json, output);
    if (!ok) {
        return false;
    }

    if (output.server.bapPort != 4321) {
        return false;
    }

    const std::string_view persona(output.steam.user.personaName.data());
    if (persona != "TestGuardian") {
        return false;
    }

    return true;
}

struct RegisterSettingsTests {
    RegisterSettingsTests() {
        register_test("Core Settings Defaults", test_settings_defaults);
        register_test("Core Settings JSON Parser Overrides", test_settings_json_parse_override);
    }
} g_register;

} // namespace
