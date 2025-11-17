use crate::app::App;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn update_search(app: &mut App) {
    if app.search_query.is_empty() {
        app.filtered_indices = (0..app.files.len()).collect();
        if !app.filtered_indices.is_empty() {
            app.list_state.select(Some(0));
        }
        return;
    }

    let matcher = SkimMatcherV2::default();
    let mut scored_indices: Vec<(usize, i64)> = app
        .files
        .iter()
        .enumerate()
        .filter_map(|(i, file)| {
            matcher
                .fuzzy_match(&file.name, &app.search_query)
                .map(|score| (i, score))
        })
        .collect();

    // Sort by score (highest first)
    scored_indices.sort_by(|a, b| b.1.cmp(&a.1));

    app.filtered_indices = scored_indices.into_iter().map(|(i, _)| i).collect();

    // Reset selection to first match
    if !app.filtered_indices.is_empty() {
        app.list_state.select(Some(0));
    } else {
        app.list_state.select(None);
    }
}
