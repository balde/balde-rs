use bitflags::bitflags;

bitflags! {
    pub struct HTTPMethod: u32 {
        const NONE = 0;
        const OPTIONS = 1 << 0;
        const GET = 1 << 1;
        const HEAD = 1 << 2;
        const POST = 1 << 3;
        const PUT = 1 << 4;
        const PATCH = 1 << 5;
        const DELETE = 1 << 6;
        const ANY = 0xff;
    }
}
