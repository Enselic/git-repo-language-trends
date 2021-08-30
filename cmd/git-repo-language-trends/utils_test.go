package main

func GetTopThreeExtensionsFrom_0() {
    perform_map_to_array_test({}, get_top_three_extensions, [])
}

func TestGetTopThreeExtensionsFrom_1() {
    perform_map_to_array_test(
        {".rs": 10},
        get_top_three_extensions,
        [".rs"],
    )
}

func TestGetTopThreeExtensionsFrom_2() {
    perform_map_to_array_test(
        {".rs": 10, ".foo": 100},
        get_top_three_extensions,
        [".foo", ".rs"],
    )
}

func TestGetTopThreeExtensionsFrom_3() {
    perform_map_to_array_test(
        {".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )
}

func TestGetTopThreeExtensionsFrom_4() {
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )
}

func TestGetTopThreeExtensionsButLockExtIsIgnored() {
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".lock": 1000},
        get_top_three_extensions,
        [".rs", ".foo", ".md"],
    )
}

func TestGetExtensionsSortedByPopularity() {
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_extensions_sorted_by_popularity,
        [".a", ".rs", ".foo", ".md"],
    )
}

// Helper function that tests that input hash map entries are transformed
// to the expected result
func PerformMapToArrayTest(input_map_entries, transformer, expected_output_entries) {
    result = transformer(input_map_entries)
    assert result == expected_output_entries
}
