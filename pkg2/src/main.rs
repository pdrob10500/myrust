
fn build_vec() -> Vec<(u8,u8)>{
    let mut v = Vec::new();
    v.push((1_000_000_u32 as u8, b'\r'));
    v.push((0xA, b'\n'));

    assert_eq!( 0xA_u8 as u16, 10_u16 );
    assert_eq!(1_000_000_u32 as u8, 64); // u32 truncated to u8

    v
}


fn main() -> Result<(), std::io::Error> {
    let ret = build_vec();
    println!("{:#?}", ret);
    Ok(())
}


