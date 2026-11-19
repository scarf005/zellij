//! Decoding of kitty unicode placeholders: cells holding U+10EEEE, whose foreground colour names an
//! image, whose underline colour names a placement, and whose combining marks give the row and
//! column of the image piece to show there.

use crate::panes::terminal_character::{AnsiCode, NamedColor, TerminalCharacter};

pub const PLACEHOLDER: char = '\u{10EEEE}';

/// The marks that stand for the numbers 0, 1, 2, ... (kitty's rowcolumn-diacritics.txt).
const DIACRITICS: [u32; 297] = [
    0x0305, 0x030D, 0x030E, 0x0310, 0x0312, 0x033D, 0x033E, 0x033F, 0x0346, 0x034A, 0x034B, 0x034C,
    0x0350, 0x0351, 0x0352, 0x0357, 0x035B, 0x0363, 0x0364, 0x0365, 0x0366, 0x0367, 0x0368, 0x0369,
    0x036A, 0x036B, 0x036C, 0x036D, 0x036E, 0x036F, 0x0483, 0x0484, 0x0485, 0x0486, 0x0487, 0x0592,
    0x0593, 0x0594, 0x0595, 0x0597, 0x0598, 0x0599, 0x059C, 0x059D, 0x059E, 0x059F, 0x05A0, 0x05A1,
    0x05A8, 0x05A9, 0x05AB, 0x05AC, 0x05AF, 0x05C4, 0x0610, 0x0611, 0x0612, 0x0613, 0x0614, 0x0615,
    0x0616, 0x0617, 0x0657, 0x0658, 0x0659, 0x065A, 0x065B, 0x065D, 0x065E, 0x06D6, 0x06D7, 0x06D8,
    0x06D9, 0x06DA, 0x06DB, 0x06DC, 0x06DF, 0x06E0, 0x06E1, 0x06E2, 0x06E4, 0x06E7, 0x06E8, 0x06EB,
    0x06EC, 0x0730, 0x0732, 0x0733, 0x0735, 0x0736, 0x073A, 0x073D, 0x073F, 0x0740, 0x0741, 0x0743,
    0x0745, 0x0747, 0x0749, 0x074A, 0x07EB, 0x07EC, 0x07ED, 0x07EE, 0x07EF, 0x07F0, 0x07F1, 0x07F3,
    0x0816, 0x0817, 0x0818, 0x0819, 0x081B, 0x081C, 0x081D, 0x081E, 0x081F, 0x0820, 0x0821, 0x0822,
    0x0823, 0x0825, 0x0826, 0x0827, 0x0829, 0x082A, 0x082B, 0x082C, 0x082D, 0x0951, 0x0953, 0x0954,
    0x0F82, 0x0F83, 0x0F86, 0x0F87, 0x135D, 0x135E, 0x135F, 0x17DD, 0x193A, 0x1A17, 0x1A75, 0x1A76,
    0x1A77, 0x1A78, 0x1A79, 0x1A7A, 0x1A7B, 0x1A7C, 0x1B6B, 0x1B6D, 0x1B6E, 0x1B6F, 0x1B70, 0x1B71,
    0x1B72, 0x1B73, 0x1CD0, 0x1CD1, 0x1CD2, 0x1CDA, 0x1CDB, 0x1CE0, 0x1DC0, 0x1DC1, 0x1DC3, 0x1DC4,
    0x1DC5, 0x1DC6, 0x1DC7, 0x1DC8, 0x1DC9, 0x1DCB, 0x1DCC, 0x1DD1, 0x1DD2, 0x1DD3, 0x1DD4, 0x1DD5,
    0x1DD6, 0x1DD7, 0x1DD8, 0x1DD9, 0x1DDA, 0x1DDB, 0x1DDC, 0x1DDD, 0x1DDE, 0x1DDF, 0x1DE0, 0x1DE1,
    0x1DE2, 0x1DE3, 0x1DE4, 0x1DE5, 0x1DE6, 0x1DFE, 0x20D0, 0x20D1, 0x20D4, 0x20D5, 0x20D6, 0x20D7,
    0x20DB, 0x20DC, 0x20E1, 0x20E7, 0x20E9, 0x20F0, 0x2CEF, 0x2CF0, 0x2CF1, 0x2DE0, 0x2DE1, 0x2DE2,
    0x2DE3, 0x2DE4, 0x2DE5, 0x2DE6, 0x2DE7, 0x2DE8, 0x2DE9, 0x2DEA, 0x2DEB, 0x2DEC, 0x2DED, 0x2DEE,
    0x2DEF, 0x2DF0, 0x2DF1, 0x2DF2, 0x2DF3, 0x2DF4, 0x2DF5, 0x2DF6, 0x2DF7, 0x2DF8, 0x2DF9, 0x2DFA,
    0x2DFB, 0x2DFC, 0x2DFD, 0x2DFE, 0x2DFF, 0xA66F, 0xA67C, 0xA67D, 0xA6F0, 0xA6F1, 0xA8E0, 0xA8E1,
    0xA8E2, 0xA8E3, 0xA8E4, 0xA8E5, 0xA8E6, 0xA8E7, 0xA8E8, 0xA8E9, 0xA8EA, 0xA8EB, 0xA8EC, 0xA8ED,
    0xA8EE, 0xA8EF, 0xA8F0, 0xA8F1, 0xAAB0, 0xAAB2, 0xAAB3, 0xAAB7, 0xAAB8, 0xAABE, 0xAABF, 0xAAC1,
    0xFE20, 0xFE21, 0xFE22, 0xFE23, 0xFE24, 0xFE25, 0xFE26, 0x10A0F, 0x10A38, 0x1D185, 0x1D186,
    0x1D187, 0x1D188, 0x1D189, 0x1D1AA, 0x1D1AB, 0x1D1AC, 0x1D1AD, 0x1D242, 0x1D243, 0x1D244,
];

pub fn diacritic_index(c: char) -> Option<u32> {
    DIACRITICS
        .binary_search(&(c as u32))
        .ok()
        .map(|index| index as u32)
}

/// A colour as the number it stands for: the 24 bit value of an RGB colour, else the palette index.
fn color_number(code: &Option<AnsiCode>) -> u32 {
    match code {
        Some(AnsiCode::RgbCode((r, g, b))) => (*r as u32) << 16 | (*g as u32) << 8 | *b as u32,
        Some(AnsiCode::ColorIndex(index)) => *index as u32,
        Some(AnsiCode::NamedColor(named)) => match named {
            NamedColor::Black => 0,
            NamedColor::Red => 1,
            NamedColor::Green => 2,
            NamedColor::Yellow => 3,
            NamedColor::Blue => 4,
            NamedColor::Magenta => 5,
            NamedColor::Cyan => 6,
            NamedColor::White => 7,
            NamedColor::BrightBlack => 8,
            NamedColor::BrightRed => 9,
            NamedColor::BrightGreen => 10,
            NamedColor::BrightYellow => 11,
            NamedColor::BrightBlue => 12,
            NamedColor::BrightMagenta => 13,
            NamedColor::BrightCyan => 14,
            NamedColor::BrightWhite => 15,
        },
        _ => 0,
    }
}

pub fn placeholder_cell(character: &TerminalCharacter) -> Option<PlaceholderCell> {
    if character.character != PLACEHOLDER {
        return None;
    }
    let text = character.text();
    let mut marks = text.chars().skip(1).filter_map(diacritic_index);
    Some(PlaceholderCell {
        image_id: color_number(&character.styles.foreground) & 0x00ff_ffff,
        placement_id: color_number(&character.styles.underline_color) & 0x00ff_ffff,
        row: marks.next(),
        column: marks.next(),
        image_id_high: marks.next(),
    })
}

/// The placeholder cells of each screen column of a line, wide characters taking more than one.
pub fn placeholder_cells<'a>(
    characters: impl Iterator<Item = &'a TerminalCharacter>,
) -> Vec<Option<PlaceholderCell>> {
    let mut cells = vec![];
    for character in characters {
        cells.push(placeholder_cell(character));
        cells.extend(std::iter::repeat(None).take(character.width().saturating_sub(1)));
    }
    cells
}

/// What one placeholder cell says on its own; missing parts are filled in from the cell on its left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceholderCell {
    /// The low 24 bits of the image id (foreground colour).
    pub image_id: u32,
    pub placement_id: u32,
    pub row: Option<u32>,
    pub column: Option<u32>,
    /// The most significant byte of the image id.
    pub image_id_high: Option<u32>,
}

/// Consecutive cells of one line showing consecutive columns of one row of one image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceholderRun {
    pub x: usize,
    pub length: usize,
    pub image_id: u32,
    pub placement_id: u32,
    pub image_row: u32,
    pub image_column: u32,
}

#[derive(Clone, Copy)]
struct Resolved {
    image_id: u32,
    placement_id: u32,
    row: u32,
    column: u32,
}

/// `cells` holds one entry per screen column of a line, `None` where there is no placeholder.
pub fn placeholder_runs(cells: &[Option<PlaceholderCell>]) -> Vec<PlaceholderRun> {
    let mut runs: Vec<PlaceholderRun> = vec![];
    let mut previous: Option<Resolved> = None;
    for (x, cell) in cells.iter().enumerate() {
        let Some(cell) = cell else {
            previous = None;
            continue;
        };
        let left = previous.filter(|left| {
            left.image_id & 0x00ff_ffff == cell.image_id && left.placement_id == cell.placement_id
        });
        let image_id = cell.image_id
            | cell
                .image_id_high
                .or_else(|| left.map(|left| left.image_id >> 24))
                .unwrap_or(0)
                << 24;
        let (row, column) = match (cell.row, cell.column, left) {
            (Some(row), Some(column), _) => (row, column),
            (Some(row), None, Some(left)) if left.row == row => (row, left.column + 1),
            (Some(row), None, _) => (row, 0),
            (None, _, Some(left)) => (left.row, left.column + 1),
            (None, _, None) => (0, 0),
        };
        previous = Some(Resolved {
            image_id,
            placement_id: cell.placement_id,
            row,
            column,
        });
        match runs.last_mut() {
            Some(run)
                if run.x + run.length == x
                    && run.image_id == image_id
                    && run.placement_id == cell.placement_id
                    && run.image_row == row
                    && run.image_column + run.length as u32 == column =>
            {
                run.length += 1
            },
            _ => runs.push(PlaceholderRun {
                x,
                length: 1,
                image_id,
                placement_id: cell.placement_id,
                image_row: row,
                image_column: column,
            }),
        }
    }
    runs
}

#[cfg(test)]
#[path = "./unit/placeholder_tests.rs"]
mod placeholder_tests;
