use configuration::account_config::{PINCODE_LENGTH, PincodeConfig};

pub const STATE_OK: u16 = 0;
pub const STATE_ASK: u16 = 1;
pub const STATE_EXPIRED: u16 = 3;
pub const STATE_NEW: u16 = 4;
pub const STATE_ILLEGAL: u16 = 5;
pub const STATE_WRONG: u16 = 8;

const SECONDS_PER_DAY: i64 = 86_400;

/// The button for PIN access was removed from clients 20180124 and later, which treat `0` as passed.
pub fn passed_state(packetver: u32) -> u16 {
    if packetver >= 20180124 { STATE_OK } else { 7 }
}

/// Reverses the keypad shuffle the client applies to the digits it sends, using the seed of the last PIN state packet.
pub fn decrypt(seed: u32, pin: &[u8]) -> Option<String> {
    if pin.len() != PINCODE_LENGTH || !pin.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let mut table: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut state = seed;
    for i in 1..10usize {
        state = 0x881234u32.wrapping_add(state.wrapping_mul(0x3498));
        let position = state as usize % (i + 1);
        table.swap(i, position);
    }
    Some(pin.iter().map(|digit| char::from(b'0' + table[usize::from(digit - b'0')])).collect())
}

pub fn allowed(config: &PincodeConfig, pin: &str) -> bool {
    let digits = pin.as_bytes();
    if digits.len() != PINCODE_LENGTH || !digits.iter().all(u8::is_ascii_digit) {
        return false;
    }
    let first = digits[0];
    if !config.allow_repeated && digits.iter().all(|digit| *digit == first) {
        return false;
    }
    if !config.allow_sequential {
        let ascending: Vec<u8> = (0..PINCODE_LENGTH as u8).map(|i| b'0' + (first - b'0' + i) % 10).collect();
        let descending: Vec<u8> = (0..PINCODE_LENGTH as u8).map(|i| b'0' + (first - b'0' + 10 - i) % 10).collect();
        if digits == ascending || digits == descending {
            return false;
        }
    }
    true
}

/// The PIN state the client is sent right after the character list.
pub fn start_state(config: &PincodeConfig, pin: &str, changed_at: i64, now: i64, verified: bool, packetver: u32) -> u16 {
    if !config.enabled {
        return STATE_OK;
    }
    if pin.is_empty() {
        return if config.force { STATE_NEW } else { passed_state(packetver) };
    }
    let lifetime = i64::from(config.changetime) * SECONDS_PER_DAY;
    if lifetime != 0 && changed_at + lifetime <= now {
        return STATE_EXPIRED;
    }
    if verified { passed_state(packetver) } else { STATE_ASK }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> PincodeConfig {
        PincodeConfig { enabled: true, ..PincodeConfig::default() }
    }

    #[test]
    fn decrypt_undoes_the_seeded_keypad_shuffle() {
        assert_eq!(decrypt(0, b"0123").unwrap(), "3061");
        assert_eq!(decrypt(1, b"9876").unwrap(), "3569");
        assert_eq!(decrypt(1234, b"4821").unwrap(), "7456");
        assert_eq!(decrypt(65534, b"0123").unwrap(), "5091");
        assert_eq!(decrypt(4_000_000_000, b"9876").unwrap(), "7032");
    }

    #[test]
    fn decrypt_rejects_anything_but_four_digits() {
        assert!(decrypt(1, b"123").is_none());
        assert!(decrypt(1, b"12345").is_none());
        assert!(decrypt(1, b"12a4").is_none());
        assert!(decrypt(1, b"").is_none());
    }

    #[test]
    fn rejects_repeated_and_sequential_pins_unless_allowed() {
        let strict = config();
        for pin in ["1111", "0000", "1234", "6789", "8901", "4321", "1098", "3210"] {
            assert!(!allowed(&strict, pin), "{pin}");
        }
        for pin in ["1235", "1243", "9090", "0159"] {
            assert!(allowed(&strict, pin), "{pin}");
        }
        assert!(allowed(&PincodeConfig { allow_repeated: true, ..config() }, "1111"));
        assert!(allowed(&PincodeConfig { allow_sequential: true, ..config() }, "1234"));
        assert!(!allowed(&strict, "12a4") && !allowed(&strict, "123"));
    }

    #[test]
    fn start_state_follows_the_enable_force_expiry_and_verified_flags() {
        let now = 1_000_000;
        assert_eq!(start_state(&PincodeConfig::default(), "1357", 0, now, false, 20120307), STATE_OK, "disabled");
        assert_eq!(start_state(&config(), "", 0, now, false, 20120307), STATE_NEW);
        let optional = PincodeConfig { force: false, ..config() };
        assert_eq!(start_state(&optional, "", 0, now, false, 20120307), 7);
        assert_eq!(start_state(&optional, "", 0, now, false, 20180124), 0);
        assert_eq!(start_state(&config(), "1357", 0, now, false, 20120307), STATE_ASK);
        assert_eq!(start_state(&config(), "1357", 0, now, true, 20120307), 7);
        let expiring = PincodeConfig { changetime: 2, ..config() };
        assert_eq!(start_state(&expiring, "1357", now - 3 * 86_400, now, true, 20120307), STATE_EXPIRED);
        assert_eq!(start_state(&expiring, "1357", now - 86_400, now, false, 20120307), STATE_ASK);
    }
}
