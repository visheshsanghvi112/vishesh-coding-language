use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;
use super::super::vastuni::Vakya;
use std::fs;
use std::ops::Deref;

pub fn lekhan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 2 {
        return Err(aadhaar.throw_dosa("लेखन (Write) २ परिमितिः अपेक्षितः - (path, content)"));
    }

    let content = aadhaar.rashi[aadhaar.rashi_len() - 1];
    let path = aadhaar.rashi[aadhaar.rashi_len() - 2];

    let path_str = match path {
        Mulya::Vakya(v) => v.deref().s.clone(),
        _ => return Err(aadhaar.throw_dosa("प्रथम परिमिति वाक्य अपेक्षितः (Path must be string)")),
    };
    
    let content_str = match content {
        Mulya::Vakya(v) => v.deref().s.clone(),
        Mulya::Ank(n) => return Err(aadhaar.throw_dosa("द्वितीय परिमिति वाक्य अपेक्षितः (Content must be string)")), 
        _ => return Err(aadhaar.throw_dosa("द्वितीय परिमिति वाक्य अपेक्षितः")),
    };

    match fs::write(path_str, content_str) {
        Ok(_) => Ok(Mulya::Satya),
        Err(_) => Err(aadhaar.throw_dosa("सञ्चिका लेखनं विफलम् (File write failed)")),
    }
}

pub fn vaachan(aadhaar: &mut Aadhaar, arg_count: usize) -> Result<Mulya, Dosa> {
    if arg_count != 1 {
        return Err(aadhaar.throw_dosa("वाचन (Read) १ परिमितिः अपेक्षितः - (path)"));
    }

    let path = aadhaar.rashi[aadhaar.rashi_len() - 1];

    let path_str = match path {
        Mulya::Vakya(v) => v.deref().s.clone(),
        _ => return Err(aadhaar.throw_dosa("प्रथम परिमिति वाक्य अपेक्षितः (Path must be string)")),
    };

    match fs::read_to_string(path_str) {
        Ok(content) => {
            // Now we HAVE access to GC through aadhaar.gc!
            // We can properly intern the string and return a GcRef<Vakya>
            let vakya_ref = aadhaar.gc.intern(content);
            Ok(Mulya::Vakya(vakya_ref))
        },
        Err(_) => Err(aadhaar.throw_dosa("सञ्चिका वाचनं विफलम् (File read failed)")),
    }
}
