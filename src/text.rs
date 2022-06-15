use wgpu::BindGroup;
use wgpu_text::font::FontRef;
use wgpu_text::section::Section;
use wgpu_text::{BrushBuilder, TextBrush};

use crate::graphics::Graphics;

pub struct Font<'a> {
    brush: TextBrush<FontRef<'a>>,
}

impl<'a> Font<'a> {
    pub fn new(graphics: &Graphics, ttf: &'a [u8]) -> Self {
        let brush = BrushBuilder::using_font_bytes(ttf)
            .unwrap()
            .build(&graphics.device, &graphics.config);

        Self {
            brush,
        }
    }

    pub fn render_text(&mut self, graphics: &Graphics, text: TextDescriptor) -> BindGroup {
        self.brush.queue(&text.section());
        todo!()
    }
}

pub struct TextDescriptor {
}

impl TextDescriptor {
    fn section(&self) -> Section {
        todo!()
    }
}
