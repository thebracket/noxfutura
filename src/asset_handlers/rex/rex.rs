// The work here is heavily derived from rs-rexpaint, https://gitlab.com/medusacle/rs-rexpaint
// It is Copyright (c) 2018, Mara <cyphergothic@protonmail.com>
// It's under the DWTFYW Public License 2.0, so inclusion in an MIT-licensed program
// isn't a problem.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

use super::XpColor;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;
use std::io;
use std::io::prelude::*;

/// Structure representing a character and its foreground/background color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpCell {
    /// Character index
    /// This depends on the font but will usually be a code page 437 character
    /// (one way to convert to a rust unicode character one way is to use
    /// `CP437_WINGDINGS.decode(...)` in the `codepage_437` crate!)
    pub ch: u32,
    /// Foreground color
    pub fg: XpColor,
    /// Background color
    pub bg: XpColor,
}

/// Structure representing a layer
/// Cells are in the same order as in the file, in column-major order (index of position x,y is y*height + x).
#[derive(Debug, Clone, PartialEq)]
pub struct XpLayer {
    /// Width of layer (in cells)
    pub width: usize,
    /// Height of layer (in cells)
    pub height: usize,
    /// Content of layer
    pub cells: Vec<XpCell>,
}

impl XpLayer {
    /// Get the cell at coordinates (x,y), or None if it is out of range.
    pub fn get(&self, x: usize, y: usize) -> Option<&XpCell> {
        if x < self.width && y < self.height {
            Some(&self.cells[x * self.height + y])
        } else {
            None
        }
    }
}

/// Structure representing a REXPaint image file which is a stack of layers
#[derive(Debug, Clone, PartialEq)]
pub struct XpFile {
    /// Version number from header
    pub version: i32,
    /// Layers of the image
    pub layers: Vec<XpLayer>,
}

impl XpFile {
    /// Read a xp image from a stream
    pub fn read<R: Read>(f: &mut R) -> io::Result<XpFile> {
        let mut rdr = GzDecoder::new(f);
        let version = rdr.read_i32::<LittleEndian>()?;
        let num_layers = rdr.read_u32::<LittleEndian>()?;

        let mut layers = Vec::<XpLayer>::new();
        layers.reserve(num_layers as usize);
        for _ in 0..num_layers {
            let width = rdr.read_u32::<LittleEndian>()? as usize;
            let height = rdr.read_u32::<LittleEndian>()? as usize;

            let mut cells = Vec::<XpCell>::new();
            cells.reserve(width * height);
            for _ in 0..width {
                // column-major order
                for _ in 0..height {
                    let ch = rdr.read_u32::<LittleEndian>()?;
                    let fg = XpColor::read(&mut rdr)?;
                    let bg = XpColor::read(&mut rdr)?;
                    cells.push(XpCell { ch, fg, bg });
                }
            }
            layers.push(XpLayer {
                width,
                height,
                cells,
            });
        }
        Ok(XpFile { version, layers })
    }
}
