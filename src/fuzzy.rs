use crate::app::App;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn update_search(app: &mut App) {
    if app.search_query.is_empty() {
        app.search_highlights.clear();
        return;
    }

    let matcher = SkimMatcherV2::default();
    let matching_indices: Vec<usize> = app
        .files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| {
            matcher
                .fuzzy_match(&file.name, &app.search_query)
                .map(|_score| i)
        })
        .collect();

    app.search_highlights = matching_indices;

    // Jump to first match if any
    if !app.search_highlights.is_empty() {
        app.list_state.select(Some(app.search_highlights[0]));
    }
}
