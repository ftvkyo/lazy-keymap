use anyhow::Result;
use svg::{node::element::{Group, Rectangle, Text}, Document};

use crate::{keyboard::Keyboard, keymap::{Keymap, KeymapLayer}};


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
    pub fn shift(mut self, s: (f32, f32)) -> Self {
        self.min_x += s.0;
        self.min_y += s.1;
        self.max_x += s.0;
        self.max_y += s.1;

        self.group = self.group
            .set("transform", format!("translate({} {})", s.0, s.1));

        self
    }
}


const FONT: &'static str = "JetBrains Mono, monospace";


fn render_svg_layer(board: &Keyboard, layer: &KeymapLayer) -> SvgGroup {
    let layer_pad = 0.2;
    let layer_stroke = 0.025;
    let layer_r = 0.1;

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

    min_x -= layer_pad;
    min_y -= layer_pad;
    max_x += layer_pad;
    max_y += layer_pad;

    let outline = Rectangle::new()
        .set("x", min_x + layer_stroke / 2.0)
        .set("y", min_y + layer_stroke / 2.0)
        .set("width", max_x - min_x - layer_stroke)
        .set("height", max_y - min_y - layer_stroke)
        .set("rx", layer_r)
        .set("ry", layer_r)
        .set("fill", "none")
        .set("stroke", "grey")
        .set("stroke-width", layer_stroke);

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


fn render_svg_name(board: &Keyboard, map: &Keymap, size: f32) -> SvgGroup {
    let name = if let Some(map_name) = &map.name {
        format!("{}: {}", board.name, map_name)
    } else {
        format!("{}", board.name)
    };

    let text = Text::new(&name)
        .set("x", 0.0)
        .set("y", 0.0)
        .set("fill", "black")
        .set("font-family", FONT)
        .set("font-size", size);

    let group = Group::new()
        .add(text);

    SvgGroup {
        group,
        min_x: 0.0,
        min_y: 0.0,
        // Note: length here is in bytes, but there can actually be many bytes per character
        max_x: name.len() as f32 * size,
        max_y: size,
    }
}


fn render_svg_layers(board: &Keyboard, map: &Keymap) -> Result<SvgGroup> {
    let layer_margin = 0.5;

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    let mut layers: Vec<SvgGroup> = Vec::new();

    for (i, layer) in map.layers.iter().enumerate() {
        let g = render_svg_layer(board, layer);

        let shift = match i {
            0 => (0.0, 0.0),
            1 => (layers[0].max_x + layer_margin, 0.0),
            2 => (0.0, layers[0].max_y + layer_margin),
            3 => (layers[0].max_x + layer_margin, layers[0].max_y + layer_margin),
            _ => return Err(anyhow::Error::msg("Can't render more than 4 layers now")),
        };

        let g = g.shift(shift);

        min_x = min_x.min(g.min_x);
        min_y = min_y.min(g.min_y);
        max_x = max_x.max(g.max_x);
        max_y = max_y.max(g.max_y);

        layers.push(g);
    }

    let mut group = Group::new();

    for layer in layers {
        group = group.add(layer.group);
    }

    Ok(SvgGroup {
        group,
        min_x,
        min_y,
        max_x,
        max_y,
    })
}


pub fn render_svg(board: &Keyboard, map: &Keymap) -> Result<Document> {
    let doc_pad = 0.1;
    let doc_name_size = 0.3;

    let name = render_svg_name(board, map, doc_name_size);
    let layers = render_svg_layers(board, map)?
        .shift((0.0, doc_name_size + doc_pad));

    let ox = name.min_x.min(layers.min_x);
    let oy = name.min_y.min(layers.min_y);
    let ow = layers.max_x.max(name.max_x) - ox;
    let oh = layers.max_y.max(name.max_y) - oy;

    let vb = (ox - doc_pad, oy - doc_pad, ow + doc_pad * 2.0, oh + doc_pad * 2.0);

    Ok(Document::new()
        .set("viewBox", vb)
        .add(name.group)
        .add(layers.group)
    )
}
