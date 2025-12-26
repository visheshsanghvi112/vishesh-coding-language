use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;
use std::ops::Deref;

// Enhanced String Module (शब्द विस्तार)
// Advanced string manipulation functions with Sanskrit names

/// शब्द (Shabd) - Convert to string (existing function)
pub fn shabd(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let arg = args.first();
    if let Some(arg) = arg {
        Ok(Mulya::Vakya(aadhaar.gc.intern(arg.to_string())))
    } else {
        Err(aadhaar.throw_dosa("Invalid argument to string Function"))
    }
}

/// कर्तन (Kartan) - Trim whitespace from both ends
pub fn kartan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("कर्तन() १ परिमितिः अपेक्षितः (requires 1 string)"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Vakya(v) => {
            let trimmed = v.deref().s.trim().to_string();
            let result = aadhaar.gc.intern(trimmed);
            Ok(Mulya::Vakya(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः वाक्य अपेक्षितः (Must be string)")),
    }
}

/// दीर्घाक्षर (Dirghakshar) - Convert to UPPERCASE
pub fn dirghakshar(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("दीर्घाक्षर() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Vakya(v) => {
            let upper = v.deref().s.to_uppercase();
            let result = aadhaar.gc.intern(upper);
            Ok(Mulya::Vakya(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः वाक्य अपेक्षितः")),
    }
}

/// ह्रस्वाक्षर (Hrasvakshar) - Convert to lowercase
pub fn hrasvakshar(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("ह्रस्वाक्षर() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Vakya(v) => {
            let lower = v.deref().s.to_lowercase();
            let result = aadhaar.gc.intern(lower);
            Ok(Mulya::Vakya(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः वाक्य अपेक्षितः")),
    }
}

/// समाविष्ट (Samavishta) - Check if string contains substring
pub fn samavishta(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("समाविष्ट() २ परिमितिः अपेक्षितः (string, substring)"));
    }

    let substring = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let main_string = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (main_string, substring) {
        (Mulya::Vakya(main), Mulya::Vakya(sub)) => {
            let contains = main.deref().s.contains(&sub.deref().s);
            Ok(if contains { Mulya::Satya } else { Mulya::Asatya })
        },
        _ => Err(aadhaar.throw_dosa("द्वौ वाक्यौ अपेक्षितौ (Both must be strings)")),
    }
}

/// प्रतिस्थापन (Pratisthapan) - Replace substring
pub fn pratisthapan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 3 {
        return Err(aadhaar.throw_dosa("प्रतिस्थापन() ३ परिमितिः अपेक्षितः (string, old, new)"));
    }

    let new_str = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let old_str = aadhaar.rashi[aadhaar.rashi_len() - 2];
    let main_string = aadhaar.rashi[aadhaar.rashi_len() - 3];

    match (main_string, old_str, new_str) {
        (Mulya::Vakya(main), Mulya::Vakya(old), Mulya::Vakya(new)) => {
            let replaced = main.deref().s.replace(&old.deref().s, &new.deref().s);
            let result = aadhaar.gc.intern(replaced);
            Ok(Mulya::Vakya(result))
        },
        _ => Err(aadhaar.throw_dosa("त्रयः वाक्याः अपेक्षिताः (All must be strings)")),
    }
}

/// अक्षरम् (Aksharam) - Get character at index
pub fn aksharam(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("अक्षरम्() २ परिमितिः अपेक्षितः (string, index)"));
    }

    let index = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let string = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (string, index) {
        (Mulya::Vakya(v), Mulya::Ank(i)) => {
            let idx = i as usize;
            let chars: Vec<char> = v.deref().s.chars().collect();
            if idx < chars.len() {
                let char_str = chars[idx].to_string();
                let result = aadhaar.gc.intern(char_str);
                Ok(Mulya::Vakya(result))
            } else {
                Err(aadhaar.throw_dosa("सूचकाङ्कः सीमातीतः (Index out of bounds)"))
            }
        },
        _ => Err(aadhaar.throw_dosa("वाक्य अङ्कः च अपेक्षितौ (String and number expected)")),
    }
}

/// विभाजन (Vibhajan) - Split string by delimiter (returns first part for simplicity)
pub fn vibhajan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("विभाजन() २ परिमितिः अपेक्षितः (string, delimiter)"));
    }

    let delimiter = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let string = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (string, delimiter) {
        (Mulya::Vakya(v), Mulya::Vakya(d)) => {
            let parts: Vec<&str> = v.deref().s.split(&d.deref().s).collect();
            // Return count of parts as a simple number (Array support would be complex)
            Ok(Mulya::Ank(parts.len() as f64))
        },
        _ => Err(aadhaar.throw_dosa("द्वौ वाक्यौ अपेक्षितौ")),
    }
}

/// उल्टा (Ulta) - Reverse string
pub fn ulta(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("उल्टा() १ परिमितिः अपेक्षितः"));
    }

    let input = aadhaar.rashi[aadhaar.rashi_len() - 1];

    match input {
        Mulya::Vakya(v) => {
            let reversed: String = v.deref().s.chars().rev().collect();
            let result = aadhaar.gc.intern(reversed);
            Ok(Mulya::Vakya(result))
        },
        _ => Err(aadhaar.throw_dosa("परिमितिः वाक्य अपेक्षितः")),
    }
}

/// आरम्भ (Aarambh) - Check if string starts with prefix
pub fn aarambh(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("आरम्भ() २ परिमितिः अपेक्षितः (string, prefix)"));
    }

    let prefix = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let string = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (string, prefix) {
        (Mulya::Vakya(s), Mulya::Vakya(p)) => {
            let starts = s.deref().s.starts_with(&p.deref().s);
            Ok(if starts { Mulya::Satya } else { Mulya::Asatya })
        },
        _ => Err(aadhaar.throw_dosa("द्वौ वाक्यौ अपेक्षितौ")),
    }
}

/// अन्त (Anta) - Check if string ends with suffix
pub fn anta(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("अन्त() २ परिमितिः अपेक्षितः (string, suffix)"));
    }

    let suffix = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let string = aadhaar.rashi[aadhaar.rashi_len() - 2];

    match (string, suffix) {
        (Mulya::Vakya(s), Mulya::Vakya(suf)) => {
            let ends = s.deref().s.ends_with(&suf.deref().s);
            Ok(if ends { Mulya::Satya } else { Mulya::Asatya })
        },
        _ => Err(aadhaar.throw_dosa("द्वौ वाक्यौ अपेक्षितौ")),
    }
}
