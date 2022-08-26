use std::path::Path;

fn main() {
    let base_path = Path::new("depends/simplest_librtmp_send264");
    let source_files = ["librtmp_send264.cpp"];

    let mut build = cc::Build::new();

    for f in source_files {
        build.file(base_path.join(Path::new(f)));
    }
    build.warnings(false);

    build.include("depends/librtmp/include");
    
    build.compile("simplest_librtmp_send264");
}
