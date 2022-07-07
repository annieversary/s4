use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() {
    let mut c = Config::default();
    *c.git_mut().sha_kind_mut() = ShaKind::Short;
    *c.build_mut().timestamp_mut() = true;
    *c.build_mut().kind_mut() = TimestampKind::DateOnly;
    vergen(c).unwrap()
}
