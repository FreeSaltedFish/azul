//! Contains functions for laying out single words (uses HarfBuzz for context-aware font shaping).
//! Right now, words are laid out on a word-per-word basis, no inter-word font shaping is done.

use webrender::api::{
    LayoutPoint, RenderApi, GlyphDimensions,
    GlyphInstance as WrGlyphInstance,
};
use app_units::Au;
use app_resources::LoadedFont;
use harfbuzz_sys::{
    hb_blob_create, hb_blob_destroy,
    hb_blob_t, hb_memory_mode_t, HB_MEMORY_MODE_READONLY,
};
use std::ptr;

// Translates to the ".codepoint" in HarfBuzz
pub type GlyphIndex = u32;
pub type GlyphInfo = GlyphIndex; // TODO: hb_info_t
pub type GlyphPosition = GlyphDimensions; // TODO: hb_position_t

pub struct ShapedWord {
    pub glyph_infos: Vec<GlyphInfo>,
    pub glyph_positions: Vec<GlyphPosition>,
}

pub struct HbFont<'a> {
    font: &'a LoadedFont,
    hb_font: *mut hb_blob_t,
}

impl<'a> HbFont<'a> {
    pub fn from_loaded_font(font: &'a LoadedFont) -> Self {
        const MEMORY_MODE: hb_memory_mode_t = HB_MEMORY_MODE_READONLY;

        // Create a HbFont with no destroy function (font is cleaned up by Rust destructor)

        let user_data_ptr = ptr::null_mut();
        let destroy_func = None;

        let font_ptr = font.font_bytes.as_ptr() as *const i8;
        let hb_font = unsafe { hb_blob_create(font_ptr, font.font_bytes.len() as u32, MEMORY_MODE, user_data_ptr, destroy_func) };
        Self {
            font,
            hb_font,
        }
    }
}

impl<'a> Drop for HbFont<'a> {
    fn drop(&mut self) {
        unsafe { hb_blob_destroy(self.hb_font) };
    }
}

pub(crate) fn shape_word(
    word: &str,
    font: &LoadedFont,
    font_size: Au,
    render_api: &RenderApi,
) -> ShapedWord {

    let font_instance_key = font.font_instances[&font_size];
    let space_glyph_indices = render_api.get_glyph_indices(font.key, word);
    let space_glyph_indices = space_glyph_indices.into_iter().filter_map(|e| e).collect::<Vec<u32>>();
    let space_glyph_dimensions = render_api.get_glyph_dimensions(font_instance_key, space_glyph_indices.clone());
    let space_glyph_dimensions = space_glyph_dimensions.into_iter().filter_map(|dim| dim).collect::<Vec<GlyphDimensions>>();

    ShapedWord {
        glyph_infos: space_glyph_indices,
        glyph_positions: space_glyph_dimensions,
    }
}

/// Return the sum of all the GlyphDimension advances.
/// Note for HarfBuzz migration: This is the "visual" word width, not the sum of the advances!
pub(crate) fn get_word_visual_width(glyph_dimensions: &[GlyphPosition]) -> f32 {
    glyph_dimensions.iter().map(|g| g.advance).sum()
}

/// Transform the indices (of the glyphs) and the dimensions to the final instances
pub(crate) fn get_glyph_instances(
    shaped_word: &ShapedWord,
) -> Vec<WrGlyphInstance> {

    let mut glyph_instances = Vec::with_capacity(shaped_word.glyph_positions.len());
    let mut current_cursor = 0.0;

    for (g_info, g_position) in shaped_word.glyph_infos.iter().zip(shaped_word.glyph_positions.iter()) {
        glyph_instances.push(WrGlyphInstance {
            index: *g_info,
            point: LayoutPoint::new(current_cursor, 0.0),
        });
        current_cursor += g_position.advance;
    }

    glyph_instances
}