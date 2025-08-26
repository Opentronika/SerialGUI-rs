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
        // Reserve space for axes labels
        let margin_left = 60.0;
        let margin_bottom = 30.0;
        let plot_size = Vec2::new(
            available_size.x - margin_left,
            (available_size.y * 0.85) - margin_bottom,
        );

        ui.horizontal(|ui| {
            // Y-axis labels area
            ui.allocate_ui_with_layout(
                Vec2::new(margin_left - 10.0, plot_size.y),
                egui::Layout::top_down(egui::Align::RIGHT),
                |ui| {
                    if !self.samples.is_empty() {
                        let min_y = self.samples.iter().cloned().fold(f32::INFINITY, f32::min);
                        let max_y = self
                            .samples
                            .iter()
                            .cloned()
                            .fold(f32::NEG_INFINITY, f32::max);
                        let y_range = max_y - min_y;
                        let padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
                        let padded_min = min_y - padding;
                        let padded_max = max_y + padding;

                        // Draw Y-axis labels
                        for i in 0..=5 {
                            let y_val =
                                padded_min + (padded_max - padded_min) * (1.0 - i as f32 / 5.0);
                            let y_pos = (plot_size.y / 5.0) * i as f32;

                            ui.allocate_new_ui(
                                egui::UiBuilder::new().max_rect(Rect::from_min_size(
                                    ui.min_rect().min + egui::vec2(0.0, y_pos - 8.0),
                                    Vec2::new(margin_left - 15.0, 16.0),
                                )),
                                |ui| {
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(format!("{y_val:.1}"));
                                        },
                                    );
                                },
                            );
                        }
                    }
                },
            );

            // Plot area
            Frame::canvas(ui.style()).show(ui, |ui| {
                ui.ctx().request_repaint();

                let (_id, rect) = ui.allocate_space(plot_size);

                // Auto-scale based on actual data
                let (min_val, max_val, x_range) = if self.samples.is_empty() {
                    (-1.0, 1.0, 0.0..=100.0)
                } else {
                    let min_y = self.samples.iter().cloned().fold(f32::INFINITY, f32::min);
                    let max_y = self
                        .samples
                        .iter()
                        .cloned()
                        .fold(f32::NEG_INFINITY, f32::max);

                    let y_range = max_y - min_y;
                    let padding = if y_range > 0.0 { y_range * 0.1 } else { 1.0 };
                    let padded_min = min_y - padding;
                    let padded_max = max_y + padding;

                    let x_max = (self.samples.len() as f32).max(10.0);

                    (padded_min, padded_max, 0.0..=x_max)
                };

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_x_y_ranges(x_range, max_val..=min_val), // Change this line
                    rect,
                );

                let mut shapes = vec![];

                // Draw grid lines
                if !self.samples.is_empty() {
                    // Horizontal grid lines
                    for i in 0..=5 {
                        let y = min_val + (max_val - min_val) * (i as f32 / 5.0);
                        let start = to_screen * pos2(0.0, y);
                        let end = to_screen * pos2(self.samples.len() as f32, y);
                        shapes.push(epaint::Shape::line_segment(
                            [start, end],
                            (0.5, Color32::GRAY.gamma_multiply(0.2)),
                        ));
                    }

                    // Vertical grid lines
                    let num_v_lines = 8;
                    for i in 0..=num_v_lines {
                        let x = (self.samples.len() as f32) * (i as f32 / num_v_lines as f32);
                        let start = to_screen * pos2(x, min_val);
                        let end = to_screen * pos2(x, max_val);
                        shapes.push(epaint::Shape::line_segment(
                            [start, end],
                            (0.5, Color32::GRAY.gamma_multiply(0.2)),
                        ));
                    }
                }

                // Draw data line
                if !self.samples.is_empty() {
                    let points: Vec<Pos2> = self
                        .samples
                        .iter()
                        .enumerate()
                        .map(|(i, &value)| to_screen * pos2(i as f32, value))
                        .collect();

                    shapes.push(epaint::Shape::line(
                        points,
                        PathStroke::new(2.0, Color32::from_rgb(0, 150, 255)),
                    ));
                }

                ui.painter().extend(shapes);
            });
        });

        // X-axis labels
        if !self.samples.is_empty() {
            ui.horizontal(|ui| {
                ui.add_space(margin_left);

                let num_x_labels = 6;
                let label_width = plot_size.x / num_x_labels as f32;

                for i in 0..=num_x_labels {
                    let x_val = (self.samples.len() as f32) * (i as f32 / num_x_labels as f32);

                    ui.allocate_ui_with_layout(
                        Vec2::new(label_width, 20.0),
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.label(format!("{x_val:.0}"));
                        },
                    );
                }
            });
        }
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
