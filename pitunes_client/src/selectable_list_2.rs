use std::iter;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, List, Text, Widget};
use unicode_width::UnicodeWidthStr;

pub struct SelectableList2<'b> {
    block: Option<Block<'b>>,
    /// Items to be displayed
    items: Vec<&'b str>,
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'b str>,
    active: Option<usize>,
    active_style: Style,
}

impl<'b> Default for SelectableList2<'b> {
    fn default() -> SelectableList2<'b> {
        SelectableList2 {
            block: None,
            items: Vec::new(),
            selected: None,
            style: Default::default(),
            highlight_symbol: None,
            active: None,
            active_style: Default::default(),
        }
    }
}

impl<'b> SelectableList2<'b> {
    pub fn block(mut self, block: Block<'b>) -> SelectableList2<'b> {
        self.block = Some(block);
        self
    }

    pub fn items<I>(mut self, items: &'b [I]) -> SelectableList2<'b>
    where
        I: AsRef<str> + 'b,
    {
        self.items = items.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
        self
    }

    pub fn style(mut self, style: Style) -> SelectableList2<'b> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'b str) -> SelectableList2<'b> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn select(mut self, index: Option<usize>) -> SelectableList2<'b> {
        self.selected = index;
        self
    }

    pub fn active_style(mut self, active_style: Style) -> SelectableList2<'b> {
        self.active_style = active_style;
        self
    }

    pub fn active(mut self, index: Option<usize>) -> SelectableList2<'b> {
        self.active = index;
        self
    }
}

impl<'b> Widget for SelectableList2<'b> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let list_area = match self.block {
            Some(ref mut b) => b.inner(area),
            None => area,
        };

        let list_height = list_area.height as usize;

        // Use active_style only if something is active
        let (active, active_style) = match self.active {
            Some(i) => (Some(i), self.active_style),
            None => (None, self.style),
        };
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = iter::repeat(" ")
            .take(highlight_symbol.width())
            .collect::<String>();
        // Make sure the list show the selected item
        let offset = if let Some(selected) = self.selected {
            if selected >= list_height {
                selected - list_height + 1
            } else {
                0
            }
        } else {
            0
        };

        // Render items
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if let Some(s) = self.selected {
                    if let Some(a) = active {
                        Text::styled(
                            format!(
                                "{} {}",
                                if i == s {
                                    highlight_symbol
                                } else {
                                    &blank_symbol[..]
                                },
                                item
                            ),
                            if i == a { active_style } else { self.style },
                        )
                    } else {
                        Text::styled(
                            format!(
                                "{} {}",
                                if i == s {
                                    highlight_symbol
                                } else {
                                    &blank_symbol[..]
                                },
                                item
                            ),
                            self.style,
                        )
                    }
                } else {
                    if let Some(a) = active {
                        Text::styled(
                            format!("{}", item),
                            if i == a { active_style } else { self.style },
                        )
                    } else {
                        Text::styled(item, self.style)
                    }
                }
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.unwrap_or_default())
            .style(self.style)
            .draw(area, buf);
    }
}
