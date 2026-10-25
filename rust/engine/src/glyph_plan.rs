//! Glyph geometry compiled once per board. Every glyph read site lies in the glyph's
//! 3×3 neighbourhood, so each glyph keeps those nine positions and the grid slot each
//! one resolved to. Slots are cached, never atom values or pointers; a rehash moves
//! every slot, so it invalidates them all and the next read links them again.

use crate::sim::{board, solution, vector};


#[derive(Clone, Copy)]
pub struct Cell {
    pub position: vector,
    /// Grid slot of `position`, or `usize::MAX` until a read links it.
    pub slot: usize,
}
pub struct Glyph {
    pub cells: [Cell; 9],
}
pub struct GlyphPlan {
    glyphs: Vec<Glyph>,
}

/// The plan for `solution`'s glyphs, built on first use. Its allocation does not move
/// while the board lives, so callers may hold the pointer across glyph reads.
pub unsafe fn plans(solution: *mut solution, board: *mut board) -> *mut Glyph {
    if (*board).glyph_plan.is_null() {
        let glyphs = (0..(*solution).number_of_glyphs)
            .map(|i| {
                let m = *(*solution).glyphs.add(i);
                Glyph {
                    cells: core::array::from_fn(|j| {
                        let (du, dv) = (j as i32 / 3 - 1, j as i32 % 3 - 1);
                        // Wrapping, as in release-mode mechanism_relative_position: only
                        // cells a glyph actually reads are observable.
                        let position = vector {
                            u: m.position.u
                                .wrapping_add(m.direction_u.u.wrapping_mul(du))
                                .wrapping_add(m.direction_v.u.wrapping_mul(dv)),
                            v: m.position.v
                                .wrapping_add(m.direction_u.v.wrapping_mul(du))
                                .wrapping_add(m.direction_v.v.wrapping_mul(dv)),
                        };
                        Cell { position, slot: usize::MAX }
                    }),
                }
            })
            .collect();
        (*board).glyph_plan = Box::into_raw(Box::new(GlyphPlan { glyphs }));
    }
    (*(*board).glyph_plan).glyphs.as_mut_ptr()
}

pub unsafe fn reset_after_rehash(board: *mut board) {
    if !(*board).glyph_plan.is_null() {
        for glyph in &mut (*(*board).glyph_plan).glyphs {
            for cell in &mut glyph.cells {
                cell.slot = usize::MAX;
            }
        }
    }
}

pub unsafe fn destroy(board: *mut board) {
    if !(*board).glyph_plan.is_null() {
        drop(Box::from_raw((*board).glyph_plan));
        (*board).glyph_plan = core::ptr::null_mut();
    }
}
