use crate::toxic::Toxic;

struct ToxicIO {
    name: String,
    address_start: u32,
    address_end: u32, // exclusive
    callback: fn (&mut Toxic) -> ()
}
