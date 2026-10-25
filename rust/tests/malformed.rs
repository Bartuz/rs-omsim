use rs_omsim::parse;

fn minimal_puzzle() -> Vec<u8> {
    let mut bytes = 3u32.to_le_bytes().to_vec();
    bytes.push(0); // name
    bytes.extend(0x0123_4567_89ab_cdefu64.to_le_bytes());
    bytes.extend(0xfedc_ba98_7654_3210u64.to_le_bytes());
    bytes.extend(0u32.to_le_bytes()); // inputs
    bytes.extend(0u32.to_le_bytes()); // outputs
    bytes.extend(1u32.to_le_bytes()); // output scale
    bytes.push(0); // no production info
    bytes
}
fn minimal_solution() -> Vec<u8> {
    let mut bytes = 7u32.to_le_bytes().to_vec();
    bytes.extend([0, 0]); // puzzle, name
    bytes.extend(0u32.to_le_bytes()); // unsolved
    bytes.extend(0u32.to_le_bytes()); // parts
    bytes
}
#[test]
fn retains_all_64_bits_unlike_upstream_read_uint64() {
    let bytes = minimal_puzzle();
    let p = parse::puzzle(&bytes).unwrap();
    assert_eq!(p.creator, 0x0123_4567_89ab_cdef);
    assert_eq!(p.parts_available, 0xfedc_ba98_7654_3210);
}
#[test]
fn every_truncated_prefix_is_rejected() {
    for bytes in [minimal_puzzle(), minimal_solution()] {
        for len in 0..bytes.len() {
            let result = if bytes[0] == 3 {
                parse::puzzle(&bytes[..len]).map(|_| ())
            } else {
                parse::solution(&bytes[..len]).map(|_| ())
            };
            assert!(result.is_err(), "accepted truncated file at {len}");
        }
    }
}
#[test]
fn rejects_hostile_lengths_before_allocating() {
    let mut string = 3u32.to_le_bytes().to_vec();
    string.extend([0xff; 20]);
    assert!(parse::puzzle(&string).is_err());
    let mut puzzle = minimal_puzzle();
    puzzle[21..25].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse::puzzle(&puzzle).is_err());
    let mut solution = minimal_solution();
    solution[10..14].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse::solution(&solution).is_err());
}
#[test]
fn strings_remain_bytes_and_trailing_extensions_are_allowed() {
    let mut bytes = minimal_solution();
    bytes.splice(4..5, [1, 255]);
    bytes.extend([9, 8, 7]);
    let solution = parse::solution(&bytes).unwrap();
    assert_eq!(solution.puzzle, &[255]);
}
#[test]
fn rejects_bad_metric_tags_and_part_versions() {
    let mut bytes = 7u32.to_le_bytes().to_vec();
    bytes.extend([0, 0]);
    bytes.extend(1u32.to_le_bytes());
    bytes.extend(9u32.to_le_bytes());
    bytes.extend([0; 40]);
    assert!(
        parse::solution(&bytes)
            .unwrap_err()
            .message
            .contains("metric tag")
    );
    let mut bytes = minimal_solution();
    bytes[10..14].copy_from_slice(&1u32.to_le_bytes());
    bytes.extend([0; 30]); // empty name followed by invalid version 0
    assert!(
        parse::solution(&bytes)
            .unwrap_err()
            .message
            .contains("part version")
    );
}
