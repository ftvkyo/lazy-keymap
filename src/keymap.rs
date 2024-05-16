use std::collections::HashMap;

use serde::Deserialize;
use svg::{node::element::{Group, Rectangle, Text}, Document};

pub type KeySlotId = String;

#[derive(Debug, Deserialize)]
pub struct KeySlot {
    /// Position of the top-left corner of the key (before rotation)
    #[serde(rename = "pos")]
    pub position: (f32, f32),
    /// Size of the key
    pub size: Option<(f32, f32)>,
    /// Rotation of the key (clockwise, degrees)
    pub angle: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Keymap {
    pub name: String,
    pub slots: HashMap<KeySlotId, KeySlot>,
}

impl Keymap {
    pub fn svg(&self) -> Document {
        let key_pad = 0.05;
        let key_stroke = 0.05;
        let key_label_size = 0.15;

        let mut min_x: f32 = 0.0;
        let mut max_x: f32 = 0.0;
        let mut min_y: f32 = 0.0;
        let mut max_y: f32 = 0.0;

        let mut rects = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", key_stroke);

        let mut texts = Group::new()
            .set("fill", "black")
            .set("font-family", "JetBrains Mono")
            .set("font-size", key_label_size);

        for (id, slot) in &self.slots {
            let size = slot.size.unwrap_or((1.0, 1.0));
            let pos = slot.position;

            let w = size.0 - key_pad * 2.0;
            let h = size.1 - key_pad * 2.0;
            let x = pos.0 + key_pad;
            let y = pos.1 + key_pad;

            let mut rect = Rectangle::new()
                .set("width", w)
                .set("height", h)
                .set("x", x)
                .set("y", y);

            let mut text = Text::new(id)
                .set("x", x + key_stroke * 2.0)
                .set("y", y + key_stroke + key_label_size);

            if let Some(angle) = slot.angle {
                let centre_x = x + w / 2.0;
                let centre_y = y + h / 2.0;
                let transform = format!("rotate({} {} {})", angle, centre_x, centre_y);

                rect = rect.set("transform", transform.clone());
                text = text.set("transform", transform.clone());
            }

            rects = rects.add(rect);
            texts = texts.add(text);

            // Naively update mins and maxs.
            // Does not account for rotation now.

            min_x = min_x.min(x);
            max_x = max_x.max(x + w + key_pad);
            min_y = min_y.min(y);
            max_y = max_y.max(y + h + key_pad);
        }

        Document::new()
            .set("viewBox", (min_x, min_y, max_x, max_y))
            .add(rects)
            .add(texts)
    }
}
