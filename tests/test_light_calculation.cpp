#include <array>
#include <cstdint>
#include <optional>
#include <span>

#include "state/equipment/light/calculation/equipment_light_calculation.h"

extern void register_test(const char* name, bool (*func)());

namespace {

bool test_light_level_evaluation_single_character() {
    using namespace sunrise::state::equipment::light;

    SlotScores character{};
    SlotScores profile{};

    // Set 8 active equipment slots with 750 light each
    for (std::size_t i = 0; i < 8; ++i) {
        character[i] = ItemScore{static_cast<std::uint16_t>(i + 1), 750};
    }

    Evaluation output{};
    bool ok = calculation::evaluate(character, profile, {}, output);
    if (!ok) {
        return false;
    }

    // 8 items * 750 = 6000 total. Divisor = 8. Average = 750.
    if (output.divisor != 8) return false;
    if (output.total != 6000) return false;
    if (output.average != 750) return false;
    if (output.averageFloat != 750.0f) return false;

    return true;
}

bool test_light_level_evaluation_cross_character_upgrade() {
    using namespace sunrise::state::equipment::light;

    SlotScores char1{};
    SlotScores profile{};
    SlotScores char2{};

    // Character 1 has 750 in slot 0
    char1[0] = ItemScore{1, 750};
    // Character 2 has higher 800 in slot 0
    char2[0] = ItemScore{2, 800};

    std::array<SlotScores, 1> otherChars = {char2};

    Evaluation output{};
    bool ok = calculation::evaluate(char1, profile, otherChars, output);
    if (!ok) {
        return false;
    }

    // Merged profile maximum should have adopted 800 from character 2
    if (!output.profile[0].has_value() || output.profile[0]->score != 800) {
        return false;
    }
    // Character slot remains unchanged as 750
    if (!output.character[0].has_value() || output.character[0]->score != 750) {
        return false;
    }

    return true;
}

struct RegisterLightTests {
    RegisterLightTests() {
        register_test("Light Level Calculation (Single Character)", test_light_level_evaluation_single_character);
        register_test("Light Level Calculation (Cross-Character Upgrade Merge)", test_light_level_evaluation_cross_character_upgrade);
    }
} g_register;

} // namespace
