use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, List, Text, Widget};

pub struct SelectableList2<'b> {
    block: Option<Block<'b>>,
    /// Items to be displayed
    items: Vec<&'b str>,
    /// Index of the one selected
    selected: Option<usize>,
    /// Base style of the widget
    style: Style,
    highlight_modifier: Modifier,
    active: Option<usize>,
    active_modifier: Modifier,
}

impl<'b> Default for SelectableList2<'b> {
    fn default() -> SelectableList2<'b> {
        SelectableList2 {
            block: None,
            items: Vec::new(),
            selected: None,
            style: Default::default(),
            highlight_modifier: Modifier::empty(),
            active: None,
            active_modifier: Modifier::empty(),
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

    pub fn highlight_modifier(mut self, highlight_modifier: Modifier) -> SelectableList2<'b> {
        self.highlight_modifier = highlight_modifier;
        self
    }

    pub fn select(mut self, index: Option<usize>) -> SelectableList2<'b> {
        self.selected = index;
        self
    }

    pub fn active_modifier(mut self, active_modifier: Modifier) -> SelectableList2<'b> {
        self.active_modifier = active_modifier;
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
                let modifier = self.style.modifier
                    | (|| {
                        if let Some(selected) = self.selected {
                            if i == selected {
                                return self.highlight_modifier;
                            }
                        }
                        Modifier::empty()
                    })()
                    | (|| {
                        if let Some(active) = self.active {
                            if i == active {
                                return self.active_modifier;
                            }
                        }
                        Modifier::empty()
                    })();
                Text::styled(item, self.style.modifier(modifier))
            })
            .skip(offset as usize);
        List::new(items)
            .block(self.block.unwrap_or_default())
            .style(self.style)
            .draw(area, buf);
    }
}
