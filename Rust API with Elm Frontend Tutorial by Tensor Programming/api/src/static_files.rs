use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("./app/public/index.html")
}

#[get("/<file..>", rank = 5)]
fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./app/public/").join(file)).ok()
}
