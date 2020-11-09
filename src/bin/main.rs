use std::ptr;

use hex;
use libc;

use ngap_codec::ngap_codec;

fn main() {
    let ctx: *const ngap_codec::asn_codec_ctx_t = ptr::null();
    println!("{:?}", ctx);

    let mut s: *mut libc::c_void = ptr::null_mut();
    println!("{:?}", s);


    // Following is the hex-dump of the file obtained by
    // `xxd -u -c256 -ps ../oai-libngapcodec/test/aper/NGSetupRequest_2.aper`
    let ngap_data = "0015404A000004001B00084002F898000000000052400F06004D79206C6974746C6520674E420066001F01000000000002F8980001000800800000010002F8390001001881C00013880015400140";

    let ngap_data = hex::decode(ngap_data).expect("Unable to parse data as hex bytes");
    println!("{:#x?}", ngap_data);

    let buff: *const libc::c_void = ngap_data.as_ptr().cast::<libc::c_void>();
    println!("{:?}", ngap_data.as_ptr());
    println!("{:?}", buff);

    let len = ngap_data.len() as ngap_codec::size_t;
    println!("{:?}", len);

    unsafe {
        // Since `asn_DEF_Ngap_NGAP_PDU` is a `static mut` inside the library,
        // we'll have to bring this definition 'inside' the unsafe block.
        let pdutype = ngap_codec::asn_DEF_Ngap_NGAP_PDU;

        let rval = ngap_codec::aper_decode(
            ctx,
            &pdutype,
            &mut s,
            buff,
            len,
            0,
            0);
        println!("{:?}", rval);
    }
}
