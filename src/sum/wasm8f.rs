use core::ptr::{addr_of, addr_of_mut};

static mut INPUT: Vec<f64> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_in_resize8f(sz: i32) -> i32 {
    let u: usize = sz as usize;
    let mv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { mv.as_mut() };
    ov.and_then(|v: &mut Vec<_>| {
        v.resize(u, 0.0);
        v.capacity().try_into().ok()
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_in_ptr8f() -> *mut f64 {
    let pv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { pv.as_mut() };
    match ov {
        None => core::ptr::null_mut(),
        Some(mv) => mv.as_mut_ptr(),
    }
}

#[cfg(feature = "sum_std")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_std8f() -> f64 {
    let pv: *const _ = unsafe { addr_of!(INPUT) };
    let ov: Option<&Vec<_>> = unsafe { pv.as_ref() };
    ov.map(|v: &Vec<_>| {
        let s: &[f64] = v;
        super::std8f::sum(s)
    })
    .unwrap_or(f64::NAN)
}

#[cfg(feature = "sum_simd8f")]
use core::arch::wasm32::{f64x2, f64x2_add, f64x2_extract_lane, v128};

/// Computes sum of a slice `&[f64]` assuming s.len() % 2 === 0
#[cfg(feature = "sum_simd8f")]
pub fn sum_simd8f_chunk2(s: &[f64]) -> f64 {
    let chunks = s.chunks_exact(2);
    let vs = chunks.map(|s: &[f64]| {
        let f1: f64 = s.first().copied().unwrap_or_default();
        let f2: f64 = s.get(1).copied().unwrap_or_default();
        f64x2(f1, f2)
    });
    #[allow(clippy::redundant_closure)]
    let tot: v128 = vs.fold(f64x2(0.0, 0.0), |state, next| f64x2_add(state, next));
    let fa: f64 = f64x2_extract_lane::<0>(tot);
    let fb: f64 = f64x2_extract_lane::<1>(tot);
    fa + fb
}

#[cfg(feature = "sum_simd8f")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_simd8f() -> f64 {
    let pv: *const _ = unsafe { addr_of!(INPUT) };
    let ov: Option<&Vec<_>> = unsafe { pv.as_ref() };
    ov.map(|v: &Vec<_>| {
        let s: &[f64] = v;
        sum_simd8f_chunk2(s)
    })
    .unwrap_or(f64::NAN)
}
