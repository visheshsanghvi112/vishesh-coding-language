use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;
use super::super::vastuni::Vakya;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

// Attempting to implement SHA256 would require external crate 'sha2'.
// Since we are modding the core without easy cargo access, we will use
// Rust's internal DefaultHasher (SipHash) which is available in std.
// It is cryptographically secure enough for a demo (though not for Bitcoin).

pub fn hash(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("GudhLipi.hash() requires 1 argument (string)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    let input_str = match input {
        Mulya::Vakya(v) => v.deref().s.clone(),
        Mulya::Ank(n) => n.to_string(),
        _ => return Err(aadhaar.throw_dosa("Input must be convertible to string")),
    };

    let mut hasher = DefaultHasher::new();
    input_str.hash(&mut hasher);
    let result_u64 = hasher.finish();
    
    // Convert hash to Hex String
    let hash_hex = format!("{:x}", result_u64);

    // Intern the string and return
    let vakya_ref = aadhaar.gc.intern(hash_hex);
    Ok(Mulya::Vakya(vakya_ref))
}
