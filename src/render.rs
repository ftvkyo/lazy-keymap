use svg::{node::element::{Group, Rectangle, Text}, Document};

use crate::{keyboard::Keyboard, keymap::Keymap};


/// Wrapper over SVG [`Group`] that knows its size,
/// so it can be arranged next to other groups.
#[derive(Debug)]
struct SvgGroup {
    group: Group,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl SvgGroup {
    pub fn shift(mut self, x: f32, y: f32) -> Self {
        self.min_x += x;
        self.min_y += y;
        self.max_x += x;
        self.max_y += y;

        self.group = self.group
            .set("transform", format!("translate({} {})", x, y));

        self
    }

    pub fn x(&self) -> f32 {
        self.min_x
    }

    pub fn y(&self) -> f32 {
        self.min_y
    }

    pub fn w(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn h(&self) -> f32 {
        self.max_y - self.min_y
    }
}


const FONT: &'static str = "JetBrains Mono, monospace";


fn render_svg_layer(board: &Keyboard) -> SvgGroup {
    let group_pad = 0.1;
    let group_margin = 0.2;
    let group_stroke = 0.025;
    let group_r = 0.1;

    let key_pad = 0.05;
    let key_stroke = 0.05;
    let key_label_size = 0.15;

    // Track boundaries around the keys

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    let mut slots = Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", key_stroke);

    let mut labels = Group::new()
        .set("fill", "black")
        .set("font-family", FONT)
        .set("font-size", key_label_size);

    for (id, key_slot) in &board.slots {
        let size = key_slot.size.unwrap_or((1.0, 1.0));
        let pos = key_slot.position;

        let w = size.0 - key_pad * 2.0 - key_stroke;
        let h = size.1 - key_pad * 2.0 - key_stroke;
        let x = pos.0 + key_pad + key_stroke / 2.0;
        let y = pos.1 + key_pad + key_stroke / 2.0;

        let mut slot = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", w)
            .set("height", h);

        let mut label = Text::new(id)
            .set("x", x + key_stroke * 2.0)
            .set("y", y + key_stroke + key_label_size);

        if let Some(angle) = key_slot.angle {
            let centre_x = x + w / 2.0;
            let centre_y = y + h / 2.0;
            let transform = format!("rotate({} {} {})", angle, centre_x, centre_y);

            slot = slot.set("transform", transform.clone());
            label = label.set("transform", transform.clone());
        }

        slots = slots.add(slot);
        labels = labels.add(label);

        // Naively update mins and maxs.
        // Does not account for rotation now.

        min_x = min_x.min(pos.0);
        min_y = min_y.min(pos.1);
        max_x = max_x.max(pos.0 + size.0);
        max_y = max_y.max(pos.1 + size.1);
    }

    let outline = Rectangle::new()
        .set("width", max_x - min_x + group_pad * 2.0 - group_stroke)
        .set("height", max_y - min_y + group_pad * 2.0 - group_stroke)
        .set("x", min_x - group_pad + group_stroke / 2.0)
        .set("y", min_y - group_pad + group_stroke / 2.0)
        .set("rx", group_r)
        .set("ry", group_r)
        .set("fill", "none")
        .set("stroke", "grey")
        .set("stroke-width", group_stroke);

    min_x -= group_margin;
    min_y -= group_margin;
    max_x += group_margin;
    max_y += group_margin;

    let group = Group::new()
        .add(slots)
        .add(labels)
        .add(outline);

    SvgGroup {
        group,
        min_x,
        min_y,
        max_x,
        max_y,
    }
}


pub fn render_svg(board: &Keyboard, map: &Keymap) -> Document {
    let doc_pad = 0.1;
    let doc_name_size = 0.3;

    let name = if let Some(map_name) = &map.name {
        format!("{}: {}", board.name, map_name)
    } else {
        format!("{}", board.name)
    };

    let name = Text::new(name)
        .set("x", doc_pad)
        .set("y", doc_pad)
        .set("fill", "black")
        .set("font-family", FONT)
        .set("font-size", doc_name_size);

    let g = render_svg_layer(board);
    let g = g.shift(0.0, doc_pad + doc_name_size + doc_pad);

    let vb = (g.x() - doc_pad, g.y() - doc_pad, g.w() + doc_pad * 2.0, g.h() + doc_pad * 2.0);

    Document::new()
        .set("viewBox", vb)
        .add(name)
        .add(g.group)
}
