use core::ptr::{addr_of, addr_of_mut};

static mut INPUT: Vec<u8> = vec![];

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cnt_in_resize3u(sz: i32) -> i32 {
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
pub extern "C" fn cnt_in_ptr3u() -> *mut u8 {
    let pv: *mut Vec<_> = unsafe { addr_of_mut!(INPUT) };
    let ov: Option<&mut Vec<_>> = unsafe { pv.as_mut() };
    match ov {
        None => core::ptr::null_mut(),
        Some(mv) => mv.as_mut_ptr(),
    }
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn cntif3u_ge(lbi: u8) -> i32 {
    let p3: *const Vec<_> = unsafe { addr_of!(INPUT) };
    let o3: Option<&Vec<_>> = unsafe { p3.as_ref() };
    o3.map(|v: &Vec<_>| {
        let s: &[u8] = v;
        super::std3u::countif3u_ge(s, lbi)
    })
    .unwrap_or(-1)
}

#[cfg(feature = "cnt_simd")]
use core::arch::wasm32::{u64x2, v128};

#[cfg(feature = "cnt_simd")]
pub fn u2v(u: u128) -> v128 {
    let hi: u128 = u >> 64;
    let lo: u128 = u & 0xffff_ffff_ffff_ffff;
    u64x2(hi as u64, lo as u64)
}

#[cfg(feature = "cnt_simd3u")]
use core::arch::wasm32::{u8x16_bitmask, u8x16_ge, u8x16_splat};

#[cfg(feature = "cnt_simd3u")]
pub fn countif3u_ge_simd_v2cnt(v: v128, lbi: v128) -> u32 {
    let result: v128 = u8x16_ge(v, lbi);
    let mask: u16 = u8x16_bitmask(result);
    mask.count_ones()
}

#[cfg(feature = "cnt_simd3u")]
pub fn countif3u_ge_simd_chunk16(s: &[u8], lbi: u8) -> i32 {
    let lbi7: v128 = u8x16_splat(lbi);
    let chunks = s.chunks_exact(16);
    let u7s = chunks.flat_map(|s7: &[u8]| s7.try_into().ok().map(u128::from_be_bytes));
    let v7s = u7s.map(u2v);
    let u5s = v7s.map(|v7: v128| countif3u_ge_simd_v2cnt(v7, lbi7));
    let cnt: u32 = u5s.sum();
    cnt.try_into().ok().unwrap_or(-1)
}

#[cfg(feature = "cnt_simd3u")]
#[allow(unsafe_code)]
#[no_mangle]
pub fn cntif3u_ge_simd_chunk16(lbi: u8) -> i32 {
    let p3: *const Vec<_> = unsafe { addr_of!(INPUT) };
    let o3: Option<&Vec<_>> = unsafe { p3.as_ref() };
    o3.map(|v: &Vec<_>| {
        let s: &[u8] = v;
        countif3u_ge_simd_chunk16(s, lbi)
    })
    .unwrap_or(-1)
}
