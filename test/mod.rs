use lexers::html::HtmlLexer;
static TESTHTML: &'static [u8; 5815] = include_bytes!("input/test.html");

#[test]
fn check_html() {
    let bufstr = String::from_utf8_lossy(TESTHTML).into_owned();
    let mut result = String::with_capacity(bufstr.len());
    for tok in HtmlLexer::new(&bufstr) {
        result.push_str(tok.text);
    }
    assert_eq!(bufstr, result);
}

#[cfg(feature = "unstable")]
mod benches {
    extern crate test;
    use lexers::html::HtmlLexer;

    fn highlight_html_nx(n: usize, b: &mut test::Bencher) {
        let mut bufstr = String::from_utf8_lossy(super::TESTHTML).into_owned();
        for _ in 0..(n - 1) {
            bufstr.push_str(&String::from_utf8_lossy(super::TESTHTML));
        }
        b.iter(|| {
            for _ in HtmlLexer::new(&bufstr) { }
        });
    }

    #[bench]
    fn highlight_html_001x(b: &mut test::Bencher) {
        highlight_html_nx(1, b);
    }

    #[bench]
    fn highlight_html_010x(b: &mut test::Bencher) {
        highlight_html_nx(10, b);
    }

    #[bench]
    fn highlight_html_100x(b: &mut test::Bencher) {
        highlight_html_nx(100, b);
    }
}
