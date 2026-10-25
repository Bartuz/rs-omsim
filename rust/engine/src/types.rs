//! Types shared by the translated engine modules (C2Rust emitted one copy per module;
//! the copies were textually identical and are merged here).
#[allow(unused_imports)]
use core::ffi::{c_char, c_double, c_int, c_schar, c_uchar, c_uint, c_void};

pub type __compar_fn_t = Option<unsafe extern "C" fn(*const c_void, *const c_void) -> i32>;
pub type atom = u64;
pub type int32_t = i32;
pub type int64_t = i64;
pub type size_t = usize;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atom_at_position {
    pub position: vector,
    pub atom: atom,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atom_grid {
    pub atoms_at_positions: *mut atom_at_position,
    pub hash_capacity: u32,
    pub board: *mut board,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atom_ref_at_position {
    pub atom: *mut atom,
    pub position: vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct board {
    pub grid: atom_grid,
    pub area: u32,
    pub flag_reset: *mut *mut atom,
    pub flag_reset_capacity: u32,
    pub flag_reset_length: u32,
    pub cycle: u64,
    pub half_cycle: i32,
    pub movements: movement_list,
    pub moving_atoms: moving_atoms,
    pub overlapped_atoms: *mut atom_at_position,
    pub number_of_overlapped_atoms: u32,
    pub overlapped_atoms_capacity: u32,
    pub atoms_being_produced: *mut vector,
    pub number_of_atoms_being_produced: u32,
    pub atoms_being_produced_capacity: u32,
    pub molecule: molecule,
    pub rs_collision: bool,
    pub collision_location: vector,
    pub collision_reason: *const c_char,
    pub atom_grabs: [u32; 16],
    pub fails_on_wrong_output_mask: u64,
    pub fails_on_wrong_output_bonds_mask: u64,
    pub wrong_output_index: usize,
    pub overlap: u64,
    pub chain_mode: chain_mode,
    pub chain_atoms: *mut chain_atom,
    pub number_of_chain_atoms: u32,
    pub chain_atom_table: *mut u32,
    pub chain_atom_table_size: u32,
    pub chain_will_become_visible: bool,
    pub area_growth_order: growth_order,
    pub area_directions: *mut linear_area_direction,
    pub number_of_area_directions: u32,
    pub disjoint_bond_table: disjoint_bond_table,
    pub collision_checks: u64,
    pub collision_check_limit: u64,
    pub complete: bool,
    pub index: *mut crate::board_index::BoardIndex,
    pub glyph_plan: *mut crate::glyph_plan::GlyphPlan,
    /// Bounding box (min_u, max_u, min_v, max_v) of every cell ever written.
    pub written_bbox: [i32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct byte_string {
    pub bytes: *mut u8,
    pub length: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chain_atom {
    pub prev_in_list: *mut u32,
    pub next_in_list: u32,
    pub flags: u32,
    pub current_position: vector,
    pub original_position: vector,
    pub area_direction: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct chain_mode(pub u32);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conduit {
    pub glyph_index: u32,
    pub other_side_glyph_index: u32,
    pub id: u32,
    pub positions: *mut vector,
    pub number_of_positions: u32,
    pub atoms: *mut atom_at_position,
    pub disjoint_bond_table: disjoint_bond_table,
    pub molecule_lengths: *mut u32,
    pub number_of_molecules: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct disjoint_bond {
    pub from_position: vector,
    pub to_position: vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct disjoint_bond_table {
    pub bonds: *mut disjoint_bond,
    pub size: u32,
    pub count: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct eventual_behavior(pub u32);
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct growth_order(pub u32);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_output {
    pub r#type: input_output_type,
    pub atoms: *mut atom_at_position,
    pub number_of_atoms: u32,
    pub center_atom_index: u32,
    pub disjoint_bonds: *mut disjoint_bond,
    pub number_of_disjoint_bonds: u32,
    pub puzzle_index: u32,
    pub solution_index: u32,
    pub number_of_outputs: u64,
    pub outputs_per_repetition: u32,
    pub maximum_feed_rate: i32,
    pub original_atoms: *mut atom_at_position,
    pub number_of_original_atoms: u32,
    pub repetition_origin: vector,
    pub repetition_direction_u: vector,
    pub repetition_direction_v: vector,
    pub min_v: i32,
    pub max_v: i32,
    pub row_min_u: *mut i32,
    pub row_max_u: *mut i32,
    pub monomer_width: i32,
    pub output_cycles: *mut u64,
    pub number_of_output_cycles: u64,
    pub output_cycles_capacity: u64,
    pub output_cycles_end: u64,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct input_output_type(pub u32);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linear_area_direction {
    pub direction: vector,
    pub footprint_at_infinity: atom_grid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mechanism {
    pub r#type: u32,
    pub position: vector,
    pub direction_u: vector,
    pub direction_v: vector,
    pub conversion: atom,
    pub conduit_index: u32,
    pub movement: vector,
    pub arm_rotation: i32,
    pub pivot_parity: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct molecule {
    pub atoms: *mut atom_ref_at_position,
    pub cursor: usize,
    pub size: usize,
    pub capacity: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movement {
    pub number_of_atoms: usize,
    pub first_atom_index: usize,
    pub r#type: u32,
    pub base: vector,
    pub grabber_offset: vector,
    pub absolute_grab_position: vector,
    pub translation: vector,
    pub base_rotation: i32,
    pub rotation: i32,
    pub piston_extension: i32,
    pub first_chain_atom: u32,
    pub disjoint_bonds: disjoint_bond_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movement_list {
    pub movements: *mut movement,
    pub capacity: usize,
    pub length: usize,
    pub cursor: usize,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct movement_type(pub u32);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moving_atoms {
    pub atoms_at_positions: *mut atom_at_position,
    pub capacity: usize,
    pub length: usize,
    pub cursor: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_atom {
    pub r#type: u8,
    pub offset: [i8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_bond {
    pub r#type: u8,
    pub from: [i8; 2],
    pub to: [i8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_cabinet {
    pub position: [i8; 2],
    pub r#type: byte_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_conduit {
    pub starting_position_a: [i8; 2],
    pub starting_position_b: [i8; 2],
    pub number_of_hexes: u32,
    pub hexes: *mut puzzle_conduit_hex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_conduit_hex {
    pub offset: [i8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_file {
    pub name: byte_string,
    pub creator: u64,
    pub parts_available: u64,
    pub number_of_inputs: u32,
    pub inputs: *mut puzzle_molecule,
    pub number_of_outputs: u32,
    pub outputs: *mut puzzle_molecule,
    pub output_scale: u32,
    pub production_info: *mut puzzle_production_info,
    pub bytes: *mut c_void,
    pub owns_bytes: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_molecule {
    pub number_of_atoms: u32,
    pub atoms: *mut puzzle_atom,
    pub number_of_bonds: u32,
    pub bonds: *mut puzzle_bond,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_production_info {
    pub shrink_left: bool,
    pub shrink_right: bool,
    pub isolate_inputs_from_outputs: bool,
    pub number_of_cabinets: u32,
    pub cabinets: *mut puzzle_cabinet,
    pub number_of_conduits: u32,
    pub conduits: *mut puzzle_conduit,
    pub number_of_vials: u32,
    pub vials: *mut puzzle_vial,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct puzzle_vial {
    pub position: [i8; 2],
    pub style: u8,
    pub count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solution {
    pub glyphs: *mut mechanism,
    pub number_of_glyphs: usize,
    pub arms: *mut mechanism,
    pub arm_tape: *mut *mut c_char,
    pub arm_tape_length: *mut usize,
    pub arm_tape_start_cycle: *mut i64,
    pub arm_tape_halt_index: *mut usize,
    pub number_of_arms: usize,
    pub tape_period: u64,
    pub track_positions: *mut vector,
    pub track_plus_motions: *mut vector,
    pub track_minus_motions: *mut vector,
    pub track_table_size: u32,
    pub track_overlap: u64,
    pub track_self_overlap: u64,
    pub number_of_track_loops: u64,
    pub conduits: *mut conduit,
    pub number_of_conduits: usize,
    pub production: bool,
    pub cabinet_walls: *mut vector,
    pub number_of_cabinet_walls: usize,
    pub cabinet_map: [[u8; 32]; 32],
    pub cabinet_violations: u32,
    pub inputs_and_outputs: *mut input_output,
    pub number_of_inputs_and_outputs: usize,
    pub target_number_of_outputs: u64,
    pub min_visible_u: i32,
    pub max_visible_u: i32,
    pub min_visible_v: i32,
    pub max_visible_v: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solution_file {
    pub puzzle: byte_string,
    pub name: byte_string,
    pub solved: bool,
    pub cycles: u32,
    pub cost: u32,
    pub area: u32,
    pub instructions: u32,
    pub number_of_parts: u32,
    pub parts: *mut solution_part,
    pub bytes: *mut c_void,
    pub owns_bytes: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solution_hex_offset {
    pub offset: [i32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solution_instruction {
    pub index: i32,
    pub instruction: c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solution_part {
    pub name: byte_string,
    pub position: [i32; 2],
    pub size: u32,
    pub rotation: i32,
    pub which_input_or_output: u32,
    pub number_of_instructions: u32,
    pub instructions: *mut solution_instruction,
    pub number_of_track_hexes: u32,
    pub track_hexes: *mut solution_hex_offset,
    pub arm_number: u32,
    pub conduit_id: u32,
    pub number_of_conduit_hexes: u32,
    pub conduit_hexes: *mut solution_hex_offset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct steady_state {
    pub eventual_behavior: eventual_behavior,
    pub number_of_cycles: u64,
    pub number_of_outputs: u64,
    pub outputs_repeat_after_cycle: u64,
    pub area_growth_order: growth_order,
    pub linear_area_growth: u64,
    pub quadratic_area_growth: f64,
    pub directional_growth: [f64; 12],
    pub pivot_parity: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub u: i32,
    pub v: i32,
}
impl chain_mode {
    pub const DISCOVER_CHAIN: Self = Self(0);
    pub const EXTEND_CHAIN: Self = Self(1);
}
impl eventual_behavior {
    pub const EVENTUALLY_ENTERS_STEADY_STATE: Self = Self(0);
    pub const EVENTUALLY_STOPS_RUNNING: Self = Self(1);
    pub const EVENTUALLY_REACHES_CYCLE_LIMIT: Self = Self(2);
}
impl growth_order {
    pub const GROWTH_NONE: Self = Self(0);
    pub const GROWTH_LINEAR: Self = Self(1);
    pub const GROWTH_QUADRATIC: Self = Self(2);
}
impl input_output_type {
    pub const INPUT: Self = Self(1);
    pub const SINGLE_OUTPUT: Self = Self(2);
    pub const REPEATING_OUTPUT: Self = Self(4);
    pub const OUTPUT: Self = Self(6);
    pub const BLOCKED: Self = Self(8);
}
impl movement_type {
    pub const SWING_MOVEMENT: Self = Self(0);
    pub const PIVOT_MOVEMENT: Self = Self(1);
    pub const PISTON_MOVEMENT: Self = Self(2);
    pub const TRACK_MOVEMENT: Self = Self(3);
    pub const IS_PISTON: Self = Self(4);
}
