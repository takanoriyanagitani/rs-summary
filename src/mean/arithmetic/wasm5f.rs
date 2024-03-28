use core::ptr::{addr_of, addr_of_mut};

static mut INPUT: Vec<f32> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mean_arithmetic_in_resize5f(sz: i32) -> i32 {
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
pub extern "C" fn mean_arithmetic_in_ptr5f() -> *mut f32 {
    let pv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { pv.as_mut() };
    match ov {
        None => core::ptr::null_mut(),
        Some(mv) => mv.as_mut_ptr(),
    }
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mean_arithmetic5f_std_fast() -> f32 {
    let p: *const Vec<_> = unsafe { addr_of!(INPUT) };
    let o: Option<&Vec<_>> = unsafe { p.as_ref() };
    o.map(|v: &Vec<_>| {
        let s: &[f32] = v;
        super::std5f::mean_arithmetic5f_std_fast(s)
    })
    .unwrap_or(f32::NAN)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mean_arithmetic5f_std_high_precision() -> f64 {
    let p: *const Vec<_> = unsafe { addr_of!(INPUT) };
    let o: Option<&Vec<_>> = unsafe { p.as_ref() };
    o.map(|v: &Vec<_>| {
        let s: &[f32] = v;
        super::std5f::mean_arithmetic5f_std_high_precision(s)
    })
    .unwrap_or(f64::NAN)
}

#[cfg(feature = "mean_simd")]
use core::arch::wasm32::v128;

#[cfg(feature = "mean_arithmetic_simd5f")]
use core::arch::wasm32::{f32x4, f32x4_add, f32x4_extract_lane, f32x4_splat};

#[cfg(feature = "mean_arithmetic_simd5f")]
pub fn f32sum_chunk4_fast(s: &[f32]) -> f32 {
    let chunks = s.chunks_exact(4);
    let v7s = chunks.map(|s4: &[f32]| {
        let f0: f32 = s4.first().copied().unwrap_or_default();
        let f1: f32 = s4.get(1).copied().unwrap_or_default();
        let f2: f32 = s4.get(2).copied().unwrap_or_default();
        let f3: f32 = s4.get(3).copied().unwrap_or_default();
        f32x4(f0, f1, f2, f3)
    });
    let vsum: v128 = v7s.fold(f32x4_splat(0.0), |state, next| f32x4_add(state, next));
    let fa: f32 = f32x4_extract_lane::<0>(vsum);
    let fb: f32 = f32x4_extract_lane::<1>(vsum);
    let fc: f32 = f32x4_extract_lane::<2>(vsum);
    let fd: f32 = f32x4_extract_lane::<3>(vsum);
    fa + fb + fc + fd
}

#[allow(unsafe_code)]
#[cfg(feature = "mean_arithmetic_simd5f")]
pub fn f32sum_chunk4_fast_direct(s: &[f32]) -> f32 {
    let pf: *const f32 = s.as_ptr();
    let pv: *const v128 = pf as *const v128;
    let sz: usize = s.len() >> 2;
    let sv: &[v128] = unsafe { std::slice::from_raw_parts(pv, sz) };
    let tot: v128 = sv
        .iter()
        .fold(f32x4_splat(0.0), |state, next| f32x4_add(state, *next));
    let f0: f32 = f32x4_extract_lane::<0>(tot);
    let f1: f32 = f32x4_extract_lane::<1>(tot);
    let f2: f32 = f32x4_extract_lane::<2>(tot);
    let f3: f32 = f32x4_extract_lane::<3>(tot);
    f0 + f1 + f2 + f3
}

#[cfg(feature = "mean_arithmetic_simd5f")]
pub fn f32avg_arithmetic_chunk4_fast(s: &[f32]) -> f32 {
    let sum: f32 = f32sum_chunk4_fast_direct(s);
    let cnt: usize = s.len();
    match cnt {
        0 => f32::NAN,
        _ => sum / (cnt as f32),
    }
}

#[cfg(feature = "mean_arithmetic_simd5f")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mean_arithmetic5f_simd_fast() -> f32 {
    let p: *const Vec<_> = unsafe { addr_of!(INPUT) };
    let o: Option<&Vec<_>> = unsafe { p.as_ref() };
    o.map(|v: &Vec<_>| {
        let s: &[f32] = v;
        f32avg_arithmetic_chunk4_fast(s)
    })
    .unwrap_or(f32::NAN)
}
