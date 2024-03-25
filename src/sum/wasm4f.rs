use core::ptr::{addr_of, addr_of_mut};

static mut INPUT: Vec<f32> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_in_resize4f(sz: i32) -> i32 {
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
pub extern "C" fn sum_in_ptr4f() -> *mut f32 {
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
pub extern "C" fn sum_std4f() -> f32 {
    let pv: *const _ = unsafe { addr_of!(INPUT) };
    let ov: Option<&Vec<_>> = unsafe { pv.as_ref() };
    ov.map(|v: &Vec<_>| {
        let s: &[f32] = v;
        super::std4f::sum(s)
    })
    .unwrap_or(f32::NAN)
}

#[cfg(feature = "sum_simd4f")]
use core::arch::wasm32::{f32x4, f32x4_add, f32x4_extract_lane, v128};

/// Computes sum of a slice `&[f32]` assuming s.len() % 2 === 0
#[cfg(feature = "sum_simd4f")]
pub fn sum_simd4f_chunk4(s: &[f32]) -> f32 {
    let chunks = s.chunks_exact(4);
    let vs = chunks.map(|s: &[f32]| {
        let f1: f32 = s.first().copied().unwrap_or_default();
        let f2: f32 = s.get(1).copied().unwrap_or_default();
        let f3: f32 = s.get(2).copied().unwrap_or_default();
        let f4: f32 = s.get(3).copied().unwrap_or_default();
        f32x4(f1, f2, f3, f4)
    });
    let tot: v128 = vs.fold(f32x4(0.0, 0.0, 0.0, 0.0), |state, next| {
        f32x4_add(state, next)
    });
    let fa: f32 = f32x4_extract_lane::<0>(tot);
    let fb: f32 = f32x4_extract_lane::<1>(tot);
    let fc: f32 = f32x4_extract_lane::<2>(tot);
    let fd: f32 = f32x4_extract_lane::<3>(tot);
    fa + fb + fc + fd
}

#[cfg(feature = "sum_simd4f")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn sum_simd4f() -> f32 {
    let pv: *const _ = unsafe { addr_of!(INPUT) };
    let ov: Option<&Vec<_>> = unsafe { pv.as_ref() };
    ov.map(|v: &Vec<_>| {
        let s: &[f32] = v;
        sum_simd4f_chunk4(s)
    })
    .unwrap_or(f32::NAN)
}
