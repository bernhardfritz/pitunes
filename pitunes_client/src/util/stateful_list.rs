use std::mem;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub pattern: String,
    pub indices: Vec<usize>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
            pattern: String::new(),
            indices: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let state = {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        };
        StatefulList {
            state,
            indices: (0..items.len()).collect(),
            items,
            pattern: String::new(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.indices.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.indices.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn selected_index(&self) -> Option<usize> {
        let selected = self.state.selected()?;
        Some(*self.indices.get(selected)?)
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.selected_index()?)
    }

    pub fn selected_item_mut(&mut self) -> Option<&mut T> {
        let selected_index = self.selected_index()?;
        self.items.get_mut(selected_index)
    }

    pub fn update_indices(&mut self, f: &dyn Fn(&T) -> &str) -> Vec<usize> {
        let matcher = SkimMatcherV2::default();
        let mut indices_score: Vec<(usize, i64)> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| (i, matcher.fuzzy_match(f(item), &self.pattern[..])))
            .filter(|(_i, score)| score.is_some())
            .map(|(i, score)| (i, score.unwrap()))
            .collect();
        indices_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let old_indices = mem::replace(
            &mut self.indices,
            indices_score.iter().map(|(i, _score)| *i).collect(),
        );
        self.state.select(Some(0));
        old_indices
    }
}
