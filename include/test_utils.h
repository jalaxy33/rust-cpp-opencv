//! test_utils.h
//! Simple C++ test framework
#pragma once

// ------------- Include headers  ---------------

#include <iostream>
#include <vector>
#include <functional>
#include <string>

// ----------------- Test Framework -----------------

namespace TestFramework {
    static std::vector<std::pair<std::string, std::function<void()>>> tests;
    
    static void add_test(const std::string& name, std::function<void()> test) {
        tests.emplace_back(name, test);
    }
    
    static int run_all_tests() {
        int passed = 0;
        std::cout << "Running " << tests.size() << " tests...\n";
        
        for (const auto& [name, test] : tests) {
            std::cout << name << " ... ";
            try {
                test();
                std::cout << "PASS\n";
                passed++;
            } catch (const std::exception&) {
                std::cout << "FAIL\n";
            }
        }
        
        std::cout << "Result: " << passed << "/" << tests.size() << " passed\n";
        return (passed == tests.size()) ? 0 : 1;
    }
}

// ----------------- Test Macros -----------------

#define TEST(name) \
    void test_##name(); \
    static bool reg_##name = (TestFramework::add_test(#name, test_##name), true); \
    void test_##name()

#define ASSERT(condition) \
    if (!(condition)) throw std::runtime_error("Failed: " #condition);

#define ASSERT_EQ(a, b) \
    if ((a) != (b)) throw std::runtime_error("Failed: " #a " == " #b);

    
#define ASSERT_NEQ(a, b) \
    if ((a) == (b)) throw std::runtime_error("Failed: " #a " != " #b);


#define ASSERT_THROWS(expr) \
    do { \
        bool threw = false; \
        try { expr; } catch (...) { threw = true; } \
        if (!threw) throw std::runtime_error("Expected exception"); \
    } while(0)

#define TEST_MAIN() \
    int main() { return TestFramework::run_all_tests(); }