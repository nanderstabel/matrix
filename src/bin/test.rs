use std::arch::x86_64::*;

#[allow(non_camel_case_types)]
pub struct m256(pub __m256);

impl m256 {
    pub fn to_array(self) -> [f32; 8] {
        self.into()
    }

    pub fn from_array(f: [f32; 8]) -> Self {
        f.into()
    }
}

impl Clone for m256 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for m256 {}

impl From<[f32; 8]> for m256 {
    fn from(arr: [f32; 8]) -> Self {
        unsafe { core::mem::transmute(arr) }
    }
}

impl From<m256> for [f32; 8] {
    fn from(m: m256) -> Self {
        unsafe { core::mem::transmute(m) }
    }
}

pub fn fused_mul_add_m256(a: m256, b: m256, c: m256) -> m256 {
    m256(unsafe { _mm256_fmadd_ps(a.0, b.0, c.0) })
}

fn main() {
    let result = fused_mul_add_m256(
        m256::from([0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0]),
        m256::from([0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 10.0, 10.0]),
        fused_mul_add_m256(
            m256::from([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -10.0, 100.0]),
            m256::from([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0]),
            m256::from([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        ),
    )
    .to_array();

    println!("{:?}", result);
}
