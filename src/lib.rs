use image::{ImageBuffer, Rgba};
use ratatui::prelude::*;
use ratatui_image::picker::Picker;
use ratatui_image::{Image, Resize};
use soft_ratatui::embedded_graphics_unicodefonts::{mono_8x13_atlas, mono_10x20_atlas};
use soft_ratatui::{EmbeddedGraphics, RasterBackend, SoftBackend};

pub struct Ratrioshka<F>
where
    F: FnOnce(&mut Frame),
{
    draw: F,
}

impl<F> Ratrioshka<F>
where
    F: FnOnce(&mut Frame),
{
    pub fn new(f: F) -> Self {
        Self { draw: f }
    }
}

impl<F> Widget for Ratrioshka<F>
where
    F: FnOnce(&mut Frame),
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let backend = SoftBackend::<EmbeddedGraphics>::new(
            area.width * 2,
            area.height * 2,
            mono_10x20_atlas(),
            None,
            None,
        );
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        terminal.draw(self.draw).unwrap();

        let backend = terminal.backend();
        let w = backend.get_pixmap_width() as u32;
        let h = backend.get_pixmap_height() as u32;
        let rgba = backend.get_pixmap_data_as_rgba();
        let picker = Picker::from_query_stdio();
        let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(w, h, rgba).unwrap();
        let proto = picker
            .unwrap()
            .new_protocol(buffer.into(), area, Resize::Fit(None))
            .unwrap();
        Image::new(&proto).render(area, buf);
    }
}
