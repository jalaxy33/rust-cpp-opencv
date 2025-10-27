#include "test_utils.h"
#include "common.h"

#include <string>
#include <filesystem>

using namespace std;

TEST(TestThrowError)
{
    try
    {
        throw_error("Test error message");
        ASSERT(false); // Should not reach here
    }
    catch (const std::runtime_error &e)
    {
        ASSERT_EQ(std::string(e.what()), "Test error message");
    }
}


TEST(TestAssertFileExists)
{
    const std::filesystem::path existing_file(__FILE__);
    const std::filesystem::path non_existing_file("non_existing_file.txt");


    assert_file_exists(existing_file); // Should not throw

    // Test that an exception is thrown for non-existing file
    try
    {
        assert_file_exists(non_existing_file);
        ASSERT(false); // Should not reach here
    }
    catch (const std::runtime_error &e)
    {
        ASSERT_EQ(std::string(e.what()), "File does not exist: " + non_existing_file.string());
    }
}


TEST_MAIN()