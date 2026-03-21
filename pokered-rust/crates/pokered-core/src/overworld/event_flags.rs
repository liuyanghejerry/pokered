use pokered_data::event_flags::{EventFlag, EVENT_FLAGS_SIZE};

/// Bit-packed event flag storage mirroring wEventFlags from the original game.
/// 316 bytes = 2528 flag bits. Matches CheckEvent/SetEvent/ResetEvent macros.
#[derive(Debug, Clone)]
pub struct EventFlags {
    data: [u8; EVENT_FLAGS_SIZE],
}

impl Default for EventFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl EventFlags {
    pub const fn new() -> Self {
        Self {
            data: [0u8; EVENT_FLAGS_SIZE],
        }
    }

    #[inline]
    pub fn check(&self, flag: EventFlag) -> bool {
        let byte = flag.byte_offset();
        let mask = flag.bit_mask();
        self.data[byte] & mask != 0
    }

    #[inline]
    pub fn set(&mut self, flag: EventFlag) {
        let byte = flag.byte_offset();
        let mask = flag.bit_mask();
        self.data[byte] |= mask;
    }

    #[inline]
    pub fn reset(&mut self, flag: EventFlag) {
        let byte = flag.byte_offset();
        let mask = flag.bit_mask();
        self.data[byte] &= !mask;
    }

    #[inline]
    pub fn check_raw(&self, bit_index: u16) -> bool {
        let byte = (bit_index as usize) / 8;
        let mask = 1u8 << (bit_index % 8);
        if byte >= EVENT_FLAGS_SIZE {
            return false;
        }
        self.data[byte] & mask != 0
    }

    #[inline]
    pub fn set_raw(&mut self, bit_index: u16) {
        let byte = (bit_index as usize) / 8;
        let mask = 1u8 << (bit_index % 8);
        if byte < EVENT_FLAGS_SIZE {
            self.data[byte] |= mask;
        }
    }

    #[inline]
    pub fn reset_raw(&mut self, bit_index: u16) {
        let byte = (bit_index as usize) / 8;
        let mask = 1u8 << (bit_index % 8);
        if byte < EVENT_FLAGS_SIZE {
            self.data[byte] &= !mask;
        }
    }

    pub fn count_set(&self) -> u32 {
        self.data.iter().map(|b| b.count_ones()).sum()
    }

    pub fn clear_all(&mut self) {
        self.data = [0u8; EVENT_FLAGS_SIZE];
    }

    pub fn as_bytes(&self) -> &[u8; EVENT_FLAGS_SIZE] {
        &self.data
    }

    pub fn from_bytes(data: [u8; EVENT_FLAGS_SIZE]) -> Self {
        Self { data }
    }
}
