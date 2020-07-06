use unicode_width::UnicodeWidthStr;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Widget};

/// A widget to display a task progress.
///
/// # Examples:
///
/// ```
/// # use tui::widgets::{Widget, Gauge, Block, Borders};
/// # use tui::style::{Style, Color, Modifier};
/// Gauge::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .style(Style::default().fg(Color::White).bg(Color::Black).modifier(Modifier::ITALIC))
///     .percent(20);
/// ```
#[derive(Debug, Clone)]
pub struct MyGauge<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<&'a str>,
    style: Style,
}

impl<'a> Default for MyGauge<'a> {
    fn default() -> MyGauge<'a> {
        MyGauge {
            block: None,
            ratio: 0.0,
            label: None,
            style: Default::default(),
        }
    }
}

impl<'a> MyGauge<'a> {
    pub fn block(mut self, block: Block<'a>) -> MyGauge<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> MyGauge<'a> {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn ratio(mut self, ratio: f64) -> MyGauge<'a> {
        assert!(
            ratio <= 1.0 && ratio >= 0.0,
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
        self
    }

    pub fn label(mut self, string: &'a str) -> MyGauge<'a> {
        self.label = Some(string);
        self
    }

    pub fn style(mut self, style: Style) -> MyGauge<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for MyGauge<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let gauge_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };
        if gauge_area.height < 1 {
            return;
        }

        if self.style.bg != Color::Reset {
            buf.set_background(gauge_area, self.style.bg);
        }

        let center = gauge_area.height / 2 + gauge_area.top();
        let width = (f64::from(gauge_area.width) * self.ratio).round() as u16;
        let end = gauge_area.left() + width;
        for y in gauge_area.top()..gauge_area.bottom() {
            // Gauge
            for x in gauge_area.left()..end {
                buf.get_mut(x, y).set_symbol(" ");
            }

            if y == center {
                // Label
                let precent_label = format!("{}%", (self.ratio * 100.0).round());
                let label = self.label.unwrap_or(&precent_label);
                let label_width = label.width() as u16;
                let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
                buf.set_string(middle, y, label, self.style);
            }

            // Fix colors
            for x in gauge_area.left()..end {
                buf.get_mut(x, y).set_modifier(Modifier::REVERSED);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn gauge_invalid_percentage() {
        MyGauge::default().percent(110);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_upper_bound() {
        MyGauge::default().ratio(1.1);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_lower_bound() {
        MyGauge::default().ratio(-0.5);
    }
}
