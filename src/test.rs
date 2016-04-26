
#[cfg(test)]
mod benches {
    extern crate test;

    use std::fs::File;
    use std::io::Read;
    use lexer::HtmlLexer;

    #[bench]
    fn highlight_html(b: &mut test::Bencher) {
        let mut buf = Vec::new();
        File::open("test.html").unwrap().read_to_end(&mut buf).unwrap();
        let bufstr = String::from_utf8(buf).unwrap();
        b.iter(|| {
            for _tok in HtmlLexer::new(&bufstr) {
                // do nothing
            }
        });
    }
}
