use std::{io::Stdout, sync::RwLock};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    constants::ELLIPSIS,
    models::{IdName, Track},
    states::{HasPrompt, HasStatefulList, State},
};

pub fn render_top_block_and_stateful_list(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    server_url: &str,
    history: &[State],
    has_stateful_list: &mut impl HasStatefulList,
    queue_lock: Option<&RwLock<Vec<Track>>>,
) {
    render_top_block(f, chunk, server_url, history, has_stateful_list);
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(chunk);
    render_stateful_list(f, chunks[0], has_stateful_list, queue_lock);
}

pub fn render_prompt(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    has_prompt: &impl HasPrompt,
) {
    let text = {
        let mut text = String::from(has_prompt.prompt());
        text.push_str(has_prompt.answer());
        text
    };
    f.set_cursor(UnicodeWidthStr::width(&text[..]) as u16, 0);
    f.render_widget(Paragraph::new(Span::from(text)), chunk);
}

pub fn render_autocomplete_prompt<T: HasPrompt + HasStatefulList>(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    t: &mut T,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(chunk);
    render_prompt(f, chunks[0], t);
    render_stateful_list(f, chunks[1], t, None);
}

fn render_top_block(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    server_url: &str,
    history: &[State],
    has_stateful_list: &impl HasStatefulList,
) {
    let stateful_list = has_stateful_list.stateful_list();
    let title = {
        let mut title = String::from(" π @ ");
        title.push_str(server_url);
        title.push('/');
        for state in history {
            let breadcrumb = match state {
                State::Albums(albums_state) => breadcrumb(albums_state),
                State::Artists(artists_state) => breadcrumb(artists_state),
                State::Genres(genres_state) => breadcrumb(genres_state),
                State::Playlists(playlists_state) => breadcrumb(playlists_state),
                State::Tracks(tracks_state) => breadcrumb(tracks_state),
                State::Root(root_state) => breadcrumb(root_state),
                _ => None,
            };
            if let Some(breadcrumb) = breadcrumb {
                title.push_str(&breadcrumb[..]);
                title.push('/');
            }
        }
        title.push_str(&stateful_list.pattern[..]);
        title.push_str("  ");
        truncate(title, chunk.width as usize - 1, ELLIPSIS)
    };
    let top_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(&title[..]);
    f.set_cursor(
        chunk.x + UnicodeWidthStr::width(&title[..]) as u16 - 1,
        chunk.y,
    );
    f.render_widget(top_block, chunk);
}

fn render_stateful_list(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    has_stateful_list: &mut impl HasStatefulList,
    queue_lock: Option<&RwLock<Vec<Track>>>,
) {
    let stateful_list = has_stateful_list.stateful_list_mut();
    let list_items = if let Some(queue_lock) = queue_lock {
        let queue_guard = queue_lock.read().unwrap();
        if let Some(first) = queue_guard.first() {
            let mut list_items = Vec::new();
            for index in stateful_list.indices.iter() {
                if let Some(item) = stateful_list.items.get(*index) {
                    let style = if item.id() == first.id() {
                        Some(Style::default().add_modifier(Modifier::BOLD))
                    } else {
                        None
                    };
                    let span = if let Some(style) = style {
                        Span::styled(item.name(), style)
                    } else {
                        Span::from(item.name())
                    };
                    list_items.push(ListItem::new(vec![Spans::from(vec![span])]));
                }
            }
            Some(list_items)
        } else {
            None
        }
    } else {
        None
    };
    let list_items = if let Some(list_items) = list_items {
        list_items
    } else {
        let mut list_items = Vec::new();
        for index in stateful_list.indices.iter() {
            if let Some(item) = stateful_list.items.get(*index) {
                list_items.push(ListItem::new(vec![Spans::from(vec![Span::from(
                    item.name(),
                )])]));
            }
        }
        list_items
    };
    let list =
        List::new(list_items).highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(list, chunk, &mut stateful_list.state);
}

fn breadcrumb(has_stateful_list: &impl HasStatefulList) -> Option<&str> {
    let stateful_list = has_stateful_list.stateful_list();
    Some(stateful_list.selected_item()?.name())
}

fn truncate<'a>(s: String, len: usize, separator: &str) -> String {
    let s_len = UnicodeWidthStr::width(&s[..]);
    if s_len <= len {
        return s;
    }
    let h = (len - UnicodeWidthStr::width(separator)) as f64 / 2.0;
    let mut truncated = String::with_capacity(len);
    let i = {
        let mut i = h.ceil() as usize;
        while !s.is_char_boundary(i) && i > 0 {
            i = i - 1
        }
        i
    };
    truncated.push_str(&s[..i]);
    truncated.push_str(separator);
    let i = {
        let mut i = s_len - h.floor() as usize;
        while !s.is_char_boundary(i) && i < s.len() {
            i = i + 1;
        }
        i
    };
    truncated.push_str(&s[i..]);
    truncated
}
