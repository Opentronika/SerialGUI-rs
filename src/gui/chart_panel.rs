use egui::{containers::Frame, emath, epaint, epaint::PathStroke, pos2, Color32, Pos2, Rect, Vec2};
use std::collections::VecDeque;

#[derive(Default)]
pub struct ChartPanel {
    pub content: VecDeque<char>,
    #[allow(dead_code)]
    should_scroll_to_bottom: bool,
    samples: VecDeque<f32>,
    stream_buffer: String, // Buffer to accumulate partial data
}

impl ChartPanel {
    pub fn new(_max_length: usize) -> Self {
        Self {
            content: VecDeque::new(),
            should_scroll_to_bottom: false,
            samples: VecDeque::new(),
            stream_buffer: String::new(),
        }
    }

    pub fn process_rx(&mut self, message: Vec<u8>) {
        // Convert bytes to string and add to buffer
        let message_str = String::from_utf8_lossy(&message);
        self.stream_buffer.push_str(&message_str);

        // Process complete values (those followed by commas)
        while let Some(comma_pos) = self.stream_buffer.find(',') {
            // Extract the complete value (everything up to the comma)
            let complete_value = self.stream_buffer[..comma_pos].trim().to_string();

            // Remove the processed part from buffer (including the comma)
            self.stream_buffer = self.stream_buffer[comma_pos + 1..].to_string();

            // Try to parse the complete value
            if !complete_value.is_empty() {
                if let Ok(sample) = complete_value.parse::<f32>() {
                    self.samples.push_back(sample);
                }
            }
        }

        // Prevent buffer from growing indefinitely
        // Keep only the last incomplete value
        if self.stream_buffer.len() > 50 {
            // If buffer gets too long without a comma, it's probably garbage
            // Keep only the last 20 characters in case there's a valid number at the end
            let keep_from = self.stream_buffer.len().saturating_sub(20);
            self.stream_buffer = self.stream_buffer[keep_from..].to_string();
        }

        // Limit the samples size to prevent memory growth
        while self.samples.len() > 1000 {
            self.samples.pop_front();
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, available_size: Vec2, _autoscroll: bool) {
        let text_size = Vec2::new(available_size.x, available_size.y * 0.85);

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();

            let (_id, rect) = ui.allocate_space(text_size);

            // Auto-scale based on actual data
            let (min_val, max_val, x_range) = if self.samples.is_empty() {
                (-1.0, 1.0, 0.0..=100.0) // Default range when no data
            } else {
                let min_y = self.samples.iter().cloned().fold(f32::INFINITY, f32::min);
                let max_y = self
                    .samples
                    .iter()
                    .cloned()
                    .fold(f32::NEG_INFINITY, f32::max);

                // Add some padding (10%) to the Y range
                let y_range = max_y - min_y;
                let padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
                let padded_min = min_y - padding;
                let padded_max = max_y + padding;

                // X range based on number of samples
                let x_max = (self.samples.len() as f32).max(10.0);

                (padded_min, padded_max, 0.0..=x_max)
            };

            let to_screen = emath::RectTransform::from_to(
                Rect::from_x_y_ranges(x_range, min_val..=max_val),
                rect,
            );

            let mut shapes = vec![];

            if !self.samples.is_empty() {
                let points: Vec<Pos2> = self
                    .samples
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| to_screen * pos2(i as f32, value))
                    .collect();

                let thickness = 2.0;
                let color = Color32::from_rgb(0, 150, 255);
                shapes.push(epaint::Shape::line(
                    points,
                    PathStroke::new(thickness, color),
                ));
            }

            // Draw grid lines for better readability (optional)
            if !self.samples.is_empty() {
                // Horizontal grid lines
                for i in 0..=4 {
                    let y = min_val + (max_val - min_val) * (i as f32 / 4.0);
                    let start = to_screen * pos2(0.0, y);
                    let end = to_screen * pos2(self.samples.len() as f32, y);
                    shapes.push(epaint::Shape::line_segment(
                        [start, end],
                        (0.5, Color32::GRAY.gamma_multiply(0.3)),
                    ));
                }
            }

            ui.painter().extend(shapes);
        });
    }
}

impl serde::Serialize for ChartPanel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.content.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ChartPanel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = VecDeque::deserialize(deserializer)?;
        Ok(Self {
            content,
            should_scroll_to_bottom: false,
            samples: VecDeque::new(),
            stream_buffer: String::new(),
        })
    }
}
