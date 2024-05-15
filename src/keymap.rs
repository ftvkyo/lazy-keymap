use std::collections::HashMap;

use serde::Deserialize;
use svg::{node::element::{Group, Rectangle}, Document};

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

        let mut min_x: f32 = 0.0;
        let mut max_x: f32 = 0.0;
        let mut min_y: f32 = 0.0;
        let mut max_y: f32 = 0.0;

        let mut g = Group::new()
            .set("fill", "grey")
            .set("stroke", "black")
            .set("stroke-width", key_stroke);

        for (id, slot) in &self.slots {
            let size = slot.size.unwrap_or((1.0, 1.0));
            let pos = slot.position;

            let w = size.0 - key_pad * 2.0;
            let h = size.1 - key_pad * 2.0;
            let x = pos.0 + key_pad;
            let y = pos.1 + key_pad;

            let mut rect = Rectangle::new()
                .set("data-id", id.as_str())
                .set("width", w)
                .set("height", h)
                .set("x", x)
                .set("y", y);

            if let Some(angle) = slot.angle {
                let centre_x = x + w / 2.0;
                let centre_y = y + h / 2.0;

                let transform = format!("rotate({} {} {})", angle, centre_x, centre_y);
                rect = rect.set("transform", transform);
            }

            g = g.add(rect);

            // Naively update mins and maxs.
            // Does not account for rotation now.

            min_x = min_x.min(x);
            max_x = max_x.max(x + w + key_pad);
            min_y = min_y.min(y);
            max_y = max_y.max(y + h + key_pad);
        }

        Document::new()
            .set("viewBox", (min_x, min_y, max_x, max_y))
            .add(g)
    }
}
