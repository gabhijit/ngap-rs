use std::ptr;

use criterion::{criterion_group, criterion_main, Criterion};

use ngap_codec::ngap_codec;

pub fn ngap_decode_bench(c: &mut Criterion) {
    let ngap_data = "0015404A000004001B00084002F898000000000052400F06004D79206C69\
        74746C6520674E420066001F01000000000002F898000100080080000001\
        0002F8390001001881C00013880015400140";
    let ngap_data = hex::decode(ngap_data).unwrap();
    let buff: *const libc::c_void = ngap_data.as_ptr().cast::<libc::c_void>();
    let len = ngap_data.len() as ngap_codec::size_t;

    c.bench_function("libngap_decode_bench", |b| {
        b.iter(|| {
            unsafe {
                let ctx: *const ngap_codec::asn_codec_ctx_t = ptr::null();
                let mut s: *mut libc::c_void = ptr::null_mut();
                // Since `asn_DEF_Ngap_NGAP_PDU` is a `static mut` inside the library,
                // we'll have to bring this definition 'inside' the unsafe block.
                let pdutype = ngap_codec::asn_DEF_Ngap_NGAP_PDU;

                let _ = ngap_codec::aper_decode(ctx, &pdutype, &mut s, buff, len, 0, 0);
            }
        });
    });
}

criterion_group!(libngap_decode, ngap_decode_bench);
criterion_main!(libngap_decode);
