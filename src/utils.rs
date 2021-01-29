use std::collections::HashMap;

pub fn get_top_three_extensions(data: &HashMap<String, usize>) -> Vec<String> {
    let mut vec: Vec<(String, usize)> = data.clone().into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut result: Vec<String> = vec
        .into_iter()
        .map(|i| i.0)
        .filter(|ext| ".lock" != ext)
        .collect();
    result.truncate(3);
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn get_top_three_extensions_from_0() {
        let data: HashMap<String, usize> = HashMap::new();
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![];
        assert_eq!(top_three, empty);
    }

    #[test]
    fn get_top_three_extensions_from_1() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert(".rs".to_owned(), 10);
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![".rs".to_owned()];
        assert_eq!(top_three, empty);
    }

    #[test]
    fn get_top_three_extensions_from_2() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert(".rs".to_owned(), 10);
        data.insert(".foo".to_owned(), 100);
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![".foo".to_owned(), ".rs".to_owned()];
        assert_eq!(top_three, empty);
    }

    #[test]
    fn get_top_three_extensions_from_3() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert(".rs".to_owned(), 100);
        data.insert(".foo".to_owned(), 10);
        data.insert(".a".to_owned(), 1000);
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![".a".to_owned(), ".rs".to_owned(), ".foo".to_owned()];
        assert_eq!(top_three, empty);
    }

    #[test]
    fn get_top_three_extensions_from_4() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert(".md".to_owned(), 5);
        data.insert(".rs".to_owned(), 100);
        data.insert(".foo".to_owned(), 10);
        data.insert(".a".to_owned(), 1000);
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![".a".to_owned(), ".rs".to_owned(), ".foo".to_owned()];
        assert_eq!(top_three, empty);
    }

    #[test]
    fn get_top_three_extensions_but_lock_ext_is_ignored() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert(".md".to_owned(), 5);
        data.insert(".rs".to_owned(), 100);
        data.insert(".foo".to_owned(), 10);
        data.insert(".lock".to_owned(), 1000);
        let top_three = super::get_top_three_extensions(&data);
        let empty: Vec<String> = vec![".rs".to_owned(), ".foo".to_owned(), ".md".to_owned()];
        assert_eq!(top_three, empty);
    }
}
