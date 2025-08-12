use egui::{
    containers::Frame, emath, epaint, epaint::PathStroke, lerp, pos2, remap, Color32, Pos2, Rect,
    Vec2,
};
use std::collections::VecDeque;

#[derive(Default)]
pub struct ChartPanel {
    pub content: VecDeque<char>,
    #[allow(dead_code)]
    should_scroll_to_bottom: bool,
    colors: bool,
}

impl ChartPanel {
    pub fn new(_max_length: usize) -> Self {
        Self {
            content: VecDeque::new(),
            should_scroll_to_bottom: false,
            colors: true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, available_size: Vec2, _autoscroll: bool) {
        let text_size = Vec2::new(available_size.x, available_size.y * 0.85);

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            let time = ui.input(|i| i.time);

            let (_id, rect) = ui.allocate_space(text_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

            let mut shapes = vec![];

            for &mode in &[2, 3, 5] {
                let mode = mode as f64;
                let n = 120;
                let speed = 1.5;

                let points: Vec<Pos2> = (0..=n)
                    .map(|i| {
                        let t = i as f64 / (n as f64);
                        let amp = (time * speed * mode).sin() / mode;
                        let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                        to_screen * pos2(t as f32, y as f32)
                    })
                    .collect();

                let thickness = 10.0 / mode as f32;
                let color = Color32::from_rgb(
                    (255.0 * (0.5 + 0.5 * (time * speed * mode * 0.5).sin())) as u8,
                    (255.0 * (0.5 + 0.5 * (time * speed * mode * 0.7).cos())) as u8,
                    (255.0 * (0.5 + 0.5 * (time * speed * mode * 1.1).sin())) as u8,
                );
                shapes.push(epaint::Shape::line(
                    points,
                    if self.colors {
                        PathStroke::new_uv(thickness, move |rect, p| {
                            let t = remap(p.x, rect.x_range(), -1.0..=1.0).abs();
                            let center_color = Color32::from_rgb(0x5B, 0xCE, 0xFA);
                            let outer_color = Color32::from_rgb(0xF5, 0xA9, 0xB8);

                            Color32::from_rgb(
                                lerp(center_color.r() as f32..=outer_color.r() as f32, t) as u8,
                                lerp(center_color.g() as f32..=outer_color.g() as f32, t) as u8,
                                lerp(center_color.b() as f32..=outer_color.b() as f32, t) as u8,
                            )
                        })
                    } else {
                        PathStroke::new(thickness, color)
                    },
                ));
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
            colors: true,
        })
    }
}
