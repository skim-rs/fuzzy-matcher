use fuzzy_matcher::FuzzyMatcher;

#[test]
fn use_cache_off_works() {
    let options = [1, 2, 3, 4, 5, 6, 7, 11].map(|x| x.to_string());

    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().use_cache(false);
    for matching in options
        .iter()
        .filter_map(|n| matcher.fuzzy_match(n.as_str(), "1").map(|score| (n, score)))
    {
        println!("{matching:?}");
    }
}

#[test]
fn use_cache_on_works() {
    let options = [1, 2, 3, 4, 5, 6, 7, 11].map(|x| x.to_string());

    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().use_cache(true);
    for matching in options
        .iter()
        .filter_map(|n| matcher.fuzzy_match(n.as_str(), "1").map(|score| (n, score)))
    {
        println!("{matching:?}");
    }
}
