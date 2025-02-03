use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight};

/// Constants for the [`noto_sans_mono_bitmap`] crate.
pub mod font_constants {
    use super::*;

    /// Height of each character raster.
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    /// The width of each single symbol in the monospaced font.
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    /// Backup character if a desired symbol is unavailable.
    pub const BACKUP_CHAR: char = '�';

    /// Font weight to use.
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

    /// Character representing backspace.
    pub const BACKSPACE: char = '\u{0008}';
}
