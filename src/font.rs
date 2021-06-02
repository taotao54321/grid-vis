use once_cell::sync::Lazy;

pub static FONT: Lazy<rusttype::Font> = Lazy::new(|| {
    rusttype::Font::try_from_bytes(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/asset/mplus-1m-regular.ttf"
    )))
    .expect("cannot load font")
});
