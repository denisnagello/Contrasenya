use rand::Rng;
use rand::rngs::ThreadRng;

pub fn generate_password(length: usize, include_lowercase: bool, include_uppercase: bool, include_digits: bool, include_special_chars: bool) -> String {
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &[u8] = b"0123456789";
    const SPECIAL_CHARS: &[u8] = b"!@#$%^&*()_-+=<>?/{}~";

    let mut charset: Vec<u8> = Vec::new();

    if include_lowercase {
        charset.extend_from_slice(LOWERCASE);
    }
    if include_uppercase {
        charset.extend_from_slice(UPPERCASE);
    }
    if include_digits {
        charset.extend_from_slice(DIGITS);
    }
    if include_special_chars {
        charset.extend_from_slice(SPECIAL_CHARS);
    }

    let mut rng: ThreadRng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}
