use crate::app::App;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn update_search(app: &mut App) {
    if app.search_query.is_empty() {
        app.search_highlights.clear();
        app.search_match_positions.clear();
        return;
    }

    let matcher = SkimMatcherV2::default();
    let mut matching_indices = Vec::new();

    app.search_match_positions.clear();

    for (i, file) in app.files.iter().enumerate() {
        if let Some((_score, positions)) = matcher.fuzzy_indices(&file.name, &app.search_query) {
            matching_indices.push(i);
            app.search_match_positions.insert(i, positions);
        }
    }

    app.search_highlights = matching_indices;

    // Jump to first match if any
    if !app.search_highlights.is_empty() {
        app.list_state.select(Some(app.search_highlights[0]));
    }
}
