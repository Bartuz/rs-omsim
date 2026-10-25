//! Auxiliary queries over the canonical atom grid. Historical area cells stay
//! in the grid; this index only selects physically occupied slots, in grid order.
//! Created lazily for chain analysis, so finite simulations do not allocate it.

pub struct BoardIndex {
    active: Vec<u64>,
}

impl BoardIndex {
    pub fn new(slots: usize) -> Self {
        Self {
            active: vec![0; slots.div_ceil(64)],
        }
    }

    pub fn reset(&mut self, slots: usize) {
        self.active.resize(slots.div_ceil(64), 0);
        self.active.fill(0);
    }

    #[inline]
    pub fn set(&mut self, slot: usize, occupied: bool) {
        let mask = 1u64 << (slot & 63);
        if occupied {
            self.active[slot >> 6] |= mask;
        } else {
            self.active[slot >> 6] &= !mask;
        }
    }

    pub fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.active
            .iter()
            .enumerate()
            .flat_map(|(word_index, &word)| {
                let mut bits = word;
                std::iter::from_fn(move || {
                    if bits == 0 {
                        return None;
                    }
                    let bit = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    Some(word_index * 64 + bit)
                })
            })
    }
}

/// The grid is canonical. Slot indices are rebuilt whenever its allocation is
/// rehashed; no atom pointers are retained here.
pub unsafe fn occupied_slots(board: *mut crate::sim::board) -> Vec<usize> {
    let slots = 4096 + (*board).grid.hash_capacity as usize;
    if (*board).index.is_null() {
        let mut index = Box::new(BoardIndex::new(slots));
        let atoms = core::slice::from_raw_parts((*board).grid.atoms_at_positions, slots);
        for (slot, a) in atoms.iter().enumerate() {
            index.set(slot, a.atom & (1 | 1 << 27) == 1);
        }
        (*board).index = Box::into_raw(index);
    }
    (*(*board).index).indices().collect()
}

#[inline]
pub unsafe fn inserted(board: *mut crate::sim::board, a: *mut crate::sim::atom_at_position) {
    if !(*board).index.is_null() {
        let slot = a.offset_from((*board).grid.atoms_at_positions) as usize;
        (*(*board).index).set(slot, (*a).atom & (1 | 1 << 27) == 1);
    }
}

#[inline]
pub unsafe fn changed(board: *mut crate::sim::board, atom: *mut u64) {
    if (*board).index.is_null() {
        return;
    }
    // A molecule reference can also point into the overlap stack. Removing an
    // upper atom leaves its base grid slot occupied; only grid addresses count.
    let start = (*board).grid.atoms_at_positions as usize
        + core::mem::offset_of!(crate::sim::atom_at_position, atom);
    let Some(bytes) = (atom as usize).checked_sub(start) else {
        return;
    };
    let stride = core::mem::size_of::<crate::sim::atom_at_position>();
    let slot = bytes / stride;
    if bytes % stride == 0 && slot < 4096 + (*board).grid.hash_capacity as usize {
        (*(*board).index).set(slot, *atom & (1 | 1 << 27) == 1);
    }
}

pub unsafe fn reset_after_rehash(board: *mut crate::sim::board, slots: usize) {
    if !(*board).index.is_null() {
        (*(*board).index).reset(slots);
    }
}

pub unsafe fn destroy(board: *mut crate::sim::board) {
    if !(*board).index.is_null() {
        drop(Box::from_raw((*board).index));
        (*board).index = core::ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::BoardIndex;
    use std::collections::BTreeSet;

    #[test]
    fn mutations_and_rehash_reset_preserve_sorted_occupied_slots() {
        for slots in [1, 63, 64, 65, 1024, 4096] {
            let mut index = BoardIndex::new(slots);
            let mut reference = BTreeSet::new();
            let mut rng = 0x542d_dead_u32;
            for step in 0..10000 {
                rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                let slot = (rng >> 1) as usize % slots;
                let occupied = rng & 1 != 0;
                index.set(slot, occupied);
                if occupied {
                    reference.insert(slot);
                } else {
                    reference.remove(&slot);
                }
                if step % 31 == 0 {
                    assert_eq!(
                        index.indices().collect::<Vec<_>>(),
                        reference.iter().copied().collect::<Vec<_>>()
                    );
                }
            }
            index.reset(slots * 4);
            assert_eq!(index.indices().count(), 0);
            index.set(slots * 4 - 1, true);
            index.set(0, true);
            assert_eq!(index.indices().collect::<Vec<_>>(), [0, slots * 4 - 1]);
        }
    }

    #[test]
    fn translated_modules_agree_on_board_layout() {
        use crate::{collision, decode, sim, steady_state, verifier};
        let expected = (
            size_of::<sim::board>(),
            align_of::<sim::board>(),
            core::mem::offset_of!(sim::board, index),
        );
        macro_rules! check {
            ($ty:ty) => {
                assert_eq!(
                    (
                        size_of::<$ty>(),
                        align_of::<$ty>(),
                        core::mem::offset_of!($ty, index)
                    ),
                    expected
                );
            };
        }
        check!(collision::board);
        check!(decode::board);
        check!(steady_state::board);
        check!(verifier::board);
    }
}
