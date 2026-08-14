#include <cstdio>
#include <cstdlib>
#include <vector>
#include <string>

struct TestCase {
    std::string name;
    bool (*func)();
};

std::vector<TestCase>& get_test_registry() {
    static std::vector<TestCase> registry;
    return registry;
}

void register_test(const char* name, bool (*func)()) {
    get_test_registry().push_back({name, func});
}

int main() {
    std::printf("========================================\n");
    std::printf(" Running Project Sunrise Test Suite (Linux) \n");
    std::printf("========================================\n\n");

    int passed = 0;
    int failed = 0;

    for (const auto& test : get_test_registry()) {
        std::printf("[ RUN      ] %s\n", test.name.c_str());
        bool ok = false;
        try {
            ok = test.func();
        } catch (...) {
            ok = false;
        }

        if (ok) {
            std::printf("[       OK ] %s\n", test.name.c_str());
            ++passed;
        } else {
            std::printf("[  FAILED  ] %s\n", test.name.c_str());
            ++failed;
        }
    }

    std::printf("\n----------------------------------------\n");
    std::printf("Test Results: %d Passed, %d Failed, %d Total\n",
                passed, failed, passed + failed);
    std::printf("----------------------------------------\n");

    return failed == 0 ? 0 : 1;
}
