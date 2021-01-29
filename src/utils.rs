use super::ExtensionToLinesMap;

pub fn get_extensions_sorted_by_popularity(data: &ExtensionToLinesMap) -> Vec<String> {
    let mut vec: Vec<(String, usize)> = data.clone().into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec.into_iter().map(|i| i.0).collect()
}

/// Excludes some extensions very unlikely to be of interest, e.g. '.lock'
pub fn get_top_three_extensions(data: &ExtensionToLinesMap) -> Vec<String> {
    let mut result: Vec<String> = get_extensions_sorted_by_popularity(data)
        .into_iter()
        .filter(|ext| ".lock" != ext)
        .collect();
    result.truncate(3);
    result
}

#[cfg(test)]
mod tests {
    use super::super::ExtensionToLinesMap;
    use std::collections::HashMap;

    #[test]
    fn get_top_three_extensions_from_0() {
        test_map_to_array(vec![], super::get_top_three_extensions, vec![]);
    }

    #[test]
    fn get_top_three_extensions_from_1() {
        test_map_to_array(
            vec![(".rs", 10)],
            super::get_top_three_extensions,
            vec![".rs"],
        );
    }

    #[test]
    fn get_top_three_extensions_from_2() {
        test_map_to_array(
            vec![(".rs", 10), (".foo", 100)],
            super::get_top_three_extensions,
            vec![".foo", ".rs"],
        );
    }

    #[test]
    fn get_top_three_extensions_from_3() {
        test_map_to_array(
            vec![(".rs", 100), (".foo", 10), (".a", 1000)],
            super::get_top_three_extensions,
            vec![".a", ".rs", ".foo"],
        );
    }

    #[test]
    fn get_top_three_extensions_from_4() {
        test_map_to_array(
            vec![(".md", 5), (".rs", 100), (".foo", 10), (".a", 1000)],
            super::get_top_three_extensions,
            vec![".a", ".rs", ".foo"],
        );
    }

    #[test]
    fn get_top_three_extensions_but_lock_ext_is_ignored() {
        test_map_to_array(
            vec![(".md", 5), (".rs", 100), (".foo", 10), (".lock", 1000)],
            super::get_top_three_extensions,
            vec![".rs", ".foo", ".md"],
        );
    }

    #[test]
    fn get_extensions_sorted_by_popularity() {
        test_map_to_array(
            vec![(".md", 5), (".rs", 100), (".foo", 10), (".a", 1000)],
            super::get_extensions_sorted_by_popularity,
            vec![".a", ".rs", ".foo", ".md"],
        );
    }

    /// Helper function that tests that input hash map entries are transformed
    /// to the expected result
    fn test_map_to_array(
        input_map_entries: Vec<(&str, usize)>,
        transformer: fn(&ExtensionToLinesMap) -> Vec<String>,
        expected_output_entries: Vec<&str>,
    ) {
        let data = generate_test_data(input_map_entries);
        let result = transformer(&data);
        let expected: Vec<String> = expected_output_entries
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(result, expected);
    }

    fn generate_test_data(entries: Vec<(&str, usize)>) -> ExtensionToLinesMap {
        let mut data: ExtensionToLinesMap = HashMap::new();
        for entry in entries {
            data.insert(String::from(entry.0), entry.1);
        }
        data
    }
}
