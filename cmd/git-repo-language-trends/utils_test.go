package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGetTopThreeExtensionsFrom_0(t *testing.T) {
    performMapToArrayTest({}, get_top_three_extensions, [])
}

func TestGetTopThreeExtensionsFrom_1(t *testing.T) {
    performMapToArrayTest(
        {".rs": 10},
        get_top_three_extensions,
        [".rs"],
    )
}

func TestGetTopThreeExtensionsFrom_2(t *testing.T) {
    performMapToArrayTest(
        {".rs": 10, ".foo": 100},
        get_top_three_extensions,
        [".foo", ".rs"],
    )
}

func TestGetTopThreeExtensionsFrom_3(t *testing.T) {
    performMapToArrayTest(
        {".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )
}

func TestGetTopThreeExtensionsFrom_4(t *testing.T) {
    performMapToArrayTest(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_top_three_extensions,
        [".a", ".rs", ".foo"],
    )
}

func TestGetTopThreeExtensionsButLockExtIsIgnored(t *testing.T) {
    performMapToArrayTest(
        {".md": 5, ".rs": 100, ".foo": 10, ".lock": 1000},
        get_top_three_extensions,
        [".rs", ".foo", ".md"],
    )
}

func TestGetExtensionsSortedByPopularity(t *testing.T) {
    performMapToArrayTest(
        {".md": 5, ".rs": 100, ".foo": 10, ".a": 1000},
        get_extensions_sorted_by_popularity,
        [".a", ".rs", ".foo", ".md"],
    )
}

// Helper function that tests that input hash map entries are transformed
// to the expected result
func performMapToArrayTest(
	t *testing.T,
	input_map_entries map[string]int,
	 transformer func(map[string]int) []string,
	  expected_output_entries []string,
	  ) {
    result = transformer(input_map_entries)
    assert.Equals(t, result, expected_output_entries, "works")
}
