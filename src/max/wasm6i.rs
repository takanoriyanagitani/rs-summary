use core::ptr::{addr_of, addr_of_mut};

static mut INPUT: Vec<i64> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn max_in_resize6i(sz: i32) -> i32 {
    let u: usize = sz as usize;
    let mv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { mv.as_mut() };
    ov.and_then(|v: &mut Vec<_>| {
        v.resize(u, 0);
        v.capacity().try_into().ok()
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn max_in_ptr6i() -> *mut i64 {
    let pv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { pv.as_mut() };
    match ov {
        None => core::ptr::null_mut(),
        Some(mv) => mv.as_mut_ptr(),
    }
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn max6i(alt: i64) -> i64 {
    let p: *const _ = unsafe { addr_of!(INPUT) };
    let o: Option<_> = unsafe { p.as_ref() };
    o.map(|v: &_| {
        let s: &[i64] = v;
        super::std6i::max6i(s, alt)
    })
    .unwrap_or(alt)
}

#[cfg(feature = "max_simd")]
use core::arch::wasm32::{v128, v128_bitselect};

#[cfg(feature = "max_simd6i")]
use core::arch::wasm32::{i64x2, i64x2_extract_lane, i64x2_gt};

#[cfg(feature = "max_simd6i")]
pub fn max6i_chunk128simd(s: &[i64], alt: i64) -> i64 {
    let p6: *const i64 = s.as_ptr();
    let p7: *const v128 = p6 as *const v128;
    let sz: usize = s.len() >> 1;
    #[allow(unsafe_code)]
    let s7: &[v128] = unsafe { std::slice::from_raw_parts(p7, sz) };
    let init: v128 = i64x2(alt, alt);
    let max: v128 = s7.iter().fold(init, |state, next| {
        let gt: v128 = i64x2_gt(*next, state);
        v128_bitselect(*next, state, gt)
    });
    let i0: i64 = i64x2_extract_lane::<0>(max);
    let i1: i64 = i64x2_extract_lane::<1>(max);
    i0.max(i1)
}

#[cfg(feature = "max_simd6i")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn max6i_simd(alt: i64) -> i64 {
    let p: *const _ = unsafe { addr_of!(INPUT) };
    let o: Option<_> = unsafe { p.as_ref() };
    o.map(|v: &_| {
        let s: &[i64] = v;
        max6i_chunk128simd(s, alt)
    })
    .unwrap_or(alt)
}
