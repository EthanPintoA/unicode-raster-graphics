use itertools::Itertools;

use crate::Luma;

/// Maps a collection of lumas to its corresponding Unicode characters from the
/// palette.
///
/// **Note:** Usually, the width-to-height of a pixel is 1:1, while the
/// width-to-height of a character is about 1:2. This means you may have to
/// modify the lumas to achieve a square look before passing it into this
/// function. This crate doesn't handle this automatically because there're many
/// ways to do it, and sometimes it's unnecessary.
///
/// One option is to rescale the buffer with interpolation. Common if you're
/// working with an image.
///
/// Another option is to double each luma since two lumas can represent one
/// pixel.
///
/// ```
/// use unicode_raster_graphics::Luma;
/// use unicode_raster_graphics::lumas_to_unicode;
///
/// let lumas = [0, 0, 127, 127, 255, 255];
///
/// let unicode = lumas_to_unicode(lumas, 2, &[' ', '=', '#']);
///
/// assert_eq!(unicode, "  \n==\n##\n");
/// ```
pub fn lumas_to_unicode<I, L>(lumas: I, width: usize, palette: &[char]) -> String
where
	I: IntoIterator<Item = L>,
	L: Into<Luma>,
{
	let mut unicode = String::new();

	for row in &lumas.into_iter().chunks(width) {
		for luma in row {
			let ch = luma_to_char(luma.into(), palette);
			unicode.push(ch);
		}

		unicode.push('\n');
	}

	unicode
}

fn luma_to_char(luma: Luma, palette: &[char]) -> char {
	let luma = luma.luma;
	let last_idx = palette.len() - 1;

	// Converts (0 <= luma <= 255) to (0 <= idx <= last_idx)
	let idx = ((luma as f32 / 255.0) * last_idx as f32).round() as usize;
	palette[idx]
}
