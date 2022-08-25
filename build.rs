use std::path::Path;

fn main() {
    let base_path = Path::new("depends/rtmpdump-2.3/librtmp");
    let source_files = ["rtmp.c", "log.c", "amf.c", "hashswf.c", "parseurl.c"];

    let mut build = cc::Build::new();

    for f in source_files {
        build.file(base_path.join(Path::new(f)));
    }
    build.define("NO_CRYPTO", "");
    build.warnings(false);

    #[cfg(target_os = "macos")]
    build.include("/opt/homebrew/opt/openssl@1.1/include");

    build.compile("rtmp");
}
