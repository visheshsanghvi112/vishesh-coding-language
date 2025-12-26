use std::fmt::{self};
use std::ptr;

use super::aadhaar::Aadhaar;
use super::dosasucana::Dosa;
use super::mulya::Mulya;

mod std_ank;
mod std_kul;
mod std_labhyate;
mod std_nirgam;
mod std_pathana;
mod std_prakaar;
mod std_samay;
mod std_shabd;
mod std_sanchika;
mod std_crypto;
mod std_ganit;
mod std_truti;
mod std_vad;

#[derive(Clone, Copy)]
pub struct MoolSutra(pub fn(&mut Aadhaar, usize) -> Result<Mulya, Dosa>);

impl MoolSutra {
    pub fn samay() -> Self {
        MoolSutra(std_samay::samay)
    }

    pub fn vad() -> Self {
        MoolSutra(std_vad::vad)
    }

    pub fn pathana() -> Self {
        MoolSutra(std_pathana::pathana)
    }

    pub fn nirgam() -> Self {
        MoolSutra(std_nirgam::nirgam)
    }

    pub fn labhyate() -> Self {
        MoolSutra(std_labhyate::labhyate)
    }

    pub fn prakaar() -> Self {
        MoolSutra(std_prakaar::prakaar)
    }

    pub fn kul() -> Self {
        MoolSutra(std_kul::kul)
    }

    pub fn truti() -> Self {
        MoolSutra(std_truti::truti)
    }

    pub fn shabd() -> Self {
        MoolSutra(std_shabd::shabd)
    }

    pub fn ank() -> Self {
        MoolSutra(std_ank::ank)
    }

    // pub fn for_each() -> Self {
    //     MoolSutra(std_list::for_each)
    // }

    // pub fn map() -> Self {
    //     MoolSutra(std_list::map)
    // }

    pub fn lekhan() -> Self {
        MoolSutra(std_sanchika::lekhan)
    }

    pub fn vaachan() -> Self {
        MoolSutra(std_sanchika::vaachan)
    }

    pub fn hash() -> Self {
        MoolSutra(std_crypto::hash)
    }

    // Mathematics Module (गणित)
    pub fn mool() -> Self {
        MoolSutra(std_ganit::mool)
    }

    pub fn ghaat() -> Self {
        MoolSutra(std_ganit::ghaat)
    }

    pub fn nirapeksh() -> Self {
        MoolSutra(std_ganit::nirapeksh)
    }

    pub fn jya() -> Self {
        MoolSutra(std_ganit::jya)
    }

    pub fn kojya() -> Self {
        MoolSutra(std_ganit::kojya)
    }

    pub fn sparshajya() -> Self {
        MoolSutra(std_ganit::sparshajya)
    }

    pub fn pai() -> Self {
        MoolSutra(std_ganit::pai)
    }

    pub fn yadrichchhik() -> Self {
        MoolSutra(std_ganit::yadrichchhik)
    }

    pub fn poornaank() -> Self {
        MoolSutra(std_ganit::poornaank)
    }

    pub fn unnayan() -> Self {
        MoolSutra(std_ganit::unnayan)
    }

    pub fn laghuganik() -> Self {
        MoolSutra(std_ganit::laghuganik)
    }

    // String Manipulation Module (शब्द विस्तार)
    pub fn kartan() -> Self {
        MoolSutra(std_shabd::kartan)
    }

    pub fn dirghakshar() -> Self {
        MoolSutra(std_shabd::dirghakshar)
    }

    pub fn hrasvakshar() -> Self {
        MoolSutra(std_shabd::hrasvakshar)
    }

    pub fn samavishta() -> Self {
        MoolSutra(std_shabd::samavishta)
    }

    pub fn pratisthapan() -> Self {
        MoolSutra(std_shabd::pratisthapan)
    }

    pub fn aksharam() -> Self {
        MoolSutra(std_shabd::aksharam)
    }

    pub fn vibhajan() -> Self {
        MoolSutra(std_shabd::vibhajan)
    }

    pub fn ulta() -> Self {
        MoolSutra(std_shabd::ulta)
    }

    pub fn aarambh() -> Self {
        MoolSutra(std_shabd::aarambh)
    }

    pub fn anta() -> Self {
        MoolSutra(std_shabd::anta)
    }
}


impl fmt::Debug for MoolSutra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn>")
    }
}

impl PartialEq for MoolSutra {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}
