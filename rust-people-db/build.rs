fn main() {
    // Only embed the icon on Windows
    #[cfg(windows)]
    {
        embed_resource::compile("app.rc", std::iter::empty::<&str>());
    }
}
