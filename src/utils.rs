pub const KILOBYTE: usize = 1024;
pub const MEGABYTE: usize = 1024 * KILOBYTE;

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

#[test]
fn test_align_down() {
    assert_eq!(align_down(0b111111, 2), 0b111110);
    assert_eq!(align_down(0b111111, 4), 0b111100);
    assert_eq!(align_down(0b111111, 8), 0b111000);
    assert_eq!(align_down(0b111111, 16), 0b110000);
}
