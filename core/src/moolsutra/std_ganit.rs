use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

// Vedic Mathematics Module (गणित)
// Traditional Sanskrit names for mathematical operations

/// मूल (Mool) - Square Root
pub fn mool(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("मूल() १ परिमितिः अपेक्षितः (requires 1 number argument)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            if n < 0.0 {
                return Err(aadhaar.throw_dosa("ऋणात्मक संख्यायाः मूलं न भवति (Cannot sqrt negative)"));
            }
            let result = n.sqrt();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः (Argument must be a number)")),
    }
}

/// घात (Ghaat) - Power/Exponent
pub fn ghaat(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("घात() २ परिमितिः अपेक्षितः (base, exponent)"));
    }

    let exponent = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let base = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (base, exponent) {
        (Mulya::Ank(b), Mulya::Ank(e)) => {
            let result = b.powf(e);
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("द्वौ अङ्कौ अपेक्षितौ (Both arguments must be numbers)")),
    }
}

/// निरपेक्ष (Nirapeksh) - Absolute Value
pub fn nirapeksh(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("निरपेक्ष() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.abs();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// ज्या (Jya) - Sine (Traditional Sanskrit term from Aryabhata)
pub fn jya(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("ज्या() १ परिमितिः अपेक्षितः (radians)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.sin();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// कोज्या (Kojya) - Cosine (Traditional Sanskrit term)
pub fn kojya(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("कोज्या() १ परिमितिः अपेक्षितः (radians)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.cos();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// स्पर्शज्या (Sparshajya) - Tangent
pub fn sparshajya(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("स्पर्शज्या() १ परिमितिः अपेक्षितः (radians)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.tan();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// पै (Pai) - Pi constant (π = 3.14159...)
pub fn pai(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 0 {
        return Err(aadhaar.throw_dosa("पै() परिमितिः न अपेक्षितः (no arguments)"));
    }
    Ok(Mulya::Ank(std::f64::consts::PI))
}

/// यादृच्छिक (Yadrichchhik) - Random number between 0.0 and 1.0
/// Note: Uses simple XorShift for demonstration; for crypto use proper RNG
pub fn yadrichchhik(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 0 {
        return Err(aadhaar.throw_dosa("यादृच्छिक() परिमितिः न अपेक्षितः"));
    }
    
    // Simple pseudo-random using time-based seed
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    
    // XorShift algorithm for simple randomness
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    
    // Convert to 0.0-1.0 range
    let result = (x as f64) / (u64::MAX as f64);
    Ok(Mulya::Ank(result))
}

/// पूर्णाङ्क (Poornaank) - Floor (round down)
pub fn poornaank(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("पूर्णाङ्क() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.floor();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// उन्नयन (Unnayan) - Ceiling (round up)
pub fn unnayan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("उन्नयन() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            let result = n.ceil();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}

/// लघुगणक (Laghuganik) - Natural Logarithm (ln)
pub fn laghuganik(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("लघुगणक() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Ank(n) => {
            if n <= 0.0 {
                return Err(aadhaar.throw_dosa("धनात्मक संख्या अपेक्षिता (Must be positive)"));
            }
            let result = n.ln();
            Ok(Mulya::Ank(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः अङ्कः अपेक्षितः")),
    }
}
