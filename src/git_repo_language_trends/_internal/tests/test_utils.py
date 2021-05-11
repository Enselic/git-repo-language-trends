from ..utils import get_top_three_extensions, get_extensions_sorted_by_popularity


def get_top_three_extensions_from_0():
    perform_map_to_array_test({}, get_top_three_extensions, [])


def test_get_top_three_extensions_from_1():
    perform_map_to_array_test(
        {".rs": 10},
        get_top_three_extensions,
        [".rs"],
    )


def test_get_top_three_extensions_from_2():
    perform_map_to_array_test(
        {".rs": 10, ".foo": 100},
        get_top_three_extensions,
        [".foo", ".rs"],
    )


def test_get_top_three_extensions_from_3():
    perform_map_to_array_test(
        {".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )


def test_get_top_three_extensions_from_4():
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )


def test_get_top_three_extensions_but_lock_ext_is_ignored():
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".lock": 1000},
        get_top_three_extensions,
        [".rs", ".foo", ".md"],
    )


def test_get_extensions_sorted_by_popularity():
    perform_map_to_array_test(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_extensions_sorted_by_popularity,
        [".a", ".rs", ".foo", ".md"],
    )

# Helper function that tests that input hash map entries are transformed
# to the expected result


def perform_map_to_array_test(input_map_entries, transformer, expected_output_entries):
    result = transformer(input_map_entries)
    assert result == expected_output_entries
