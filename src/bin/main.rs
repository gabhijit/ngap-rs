use hex;

fn main() {
    // Following is the hex-dump of the file obtained by
    // `xxd -u -c256 -ps ../oai-libngapcodec/test/aper/NGSetupRequest_2.aper`
    let ngap_data = "0015404A000004001B00084002F898000000000052400F06004D79206C6974746C6520674E420066001F01000000000002F8980001000800800000010002F8390001001881C00013880015400140";

    let ngap_data = hex::decode(ngap_data).expect("Unable to parse data as hex bytes");

    println!("{:#x?}", ngap_data);
}
