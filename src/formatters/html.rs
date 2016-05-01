// Copyright (c) 2006-2015 by the respective authors (see AUTHORS file).
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// * Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in the
//   documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

macro_rules! w { ($out:expr, $expr:expr) => { try!($out.write($expr.as_bytes())) } }
macro_rules! wf { ($out:expr, $($tt:tt)*) => { try!(write!($out, $($tt)*)) } }

#[derive(Default)]
pub struct HtmlFormatter {
    pre_class: String,
    pre_styles: String,
    classprefix: String,
    linenos: bool,
    nowrap: bool,
    style_map: HashMap<TokenType, Rc<String>>,
}

impl HtmlFormatter {
    pub fn new() -> Self {
        HtmlFormatter {
            pre_class: String::from("highlight"),
            linenos: true,
            .. Default::default()
        }
    }

    add_option!(pre_class,   with_pre_class,   String);
    add_option!(pre_styles,  with_pre_styles,  String);
    add_option!(classprefix, with_classprefix, String);
    add_option!(linenos,     with_linenos,     bool);
    add_option!(nowrap,      with_nowrap,      bool);

    fn get_style_class(&mut self, ttype: TokenType) -> Option<Rc<String>> {
        if ttype.as_short_str() == "" {
            None
        } else {
            let prefix = &self.classprefix;
            Some(self.style_map.entry(ttype).or_insert_with(
                || Rc::new(format!("{}{}", prefix, ttype.as_short_str()))).clone())
        }
    }

    fn write_escaped<W: Write>(&self, source: &str, mut out: W) -> Result<()> {
        let mut last = 0;
        for (i, ch) in source.bytes().enumerate() {
            match ch as char {
                '<' | '>' | '&' | '"' | '\'' => {
                    w!(out, &source[last..i]);
                    let s = match ch as char {
                        '>' => "&gt;",
                        '<' => "&lt;",
                        '&' => "&amp;",
                        '"' => "&quot;",
                        '\'' => "&#39;",
                        _ => unreachable!()
                    };
                    w!(out, s);
                    last = i + 1;
                }
                _ => {}
            }
        }
        if last < source.len() {
            w!(out, &source[last..]);
        }
        Ok(())
    }

    fn format_lines<'a, I, W>(&mut self, source: I, mut out: W) -> Result<usize>
        where I: Iterator<Item=Token<'a>>, W: Write
    {
        let mut last_cls = None;
        let mut linecount = 0;
        for tok in source {
            let cls = self.get_style_class(tok.ttype);
            if tok.text.is_empty() {
                continue;
            }
            let parts = tok.text.split('\n').enumerate().collect::<Vec<_>>();
            let lastpart = parts.len() - 1;
            for (i, part) in parts {
                if cls != last_cls {
                    if last_cls.is_some() {
                        w!(out, "</span>");
                    }
                    if let Some(ref cls) = cls {
                        wf!(out, "<span class=\"{}\">", cls);
                    }
                }
                try!(self.write_escaped(part, &mut out));
                if i != lastpart {
                    if cls.is_some() {
                        w!(out, "</span>");
                    }
                    w!(out, "\n");
                    linecount += 1;
                    last_cls = None;
                }
            }
            last_cls = cls;
        }
        Ok(linecount)
    }

    fn wrap_pre<'a, I, W>(&mut self, source: I, mut out: W) -> Result<usize>
        where I: Iterator<Item=Token<'a>>, W: Write
    {
        let class = if !self.pre_class.is_empty() {
            format!(" class=\"{}\"", self.pre_class)
        } else { String::new() };
        let styles = if !self.pre_styles.is_empty() {
            format!(" style=\"{}\"", self.pre_styles)
        } else { String::new() };
        // the empty span here is to keep leading empty lines from being ignored
        // by HTML parsers
        wf!(out, "<pre{}{}><span></span>", class, styles);
        let linecount = try!(self.format_lines(source, &mut out));
        w!(out, "</pre>\n");
        Ok(linecount)
    }

    fn wrap_linenos<'a, I, W>(&mut self, source: I, mut out: W) -> Result<usize>
        where I: Iterator<Item=Token<'a>>, W: Write
    {
        let mut buffer = Vec::new();
        let linecount = try!(self.wrap_pre(source, &mut buffer));
        let class = if !self.pre_class.is_empty() {
            format!(" class=\"{}table\"", self.pre_class)
        } else { String::new() };
        wf!(out, "<table{}><tr><td class=\"linenos\"><pre>", class);
        let maxlen = format!("{}", linecount + 1).len();
        for line in 1..linecount + 1 {
            wf!(out, "{:1$}\n", line, maxlen);
        }
        wf!(out, "</pre>\n</td><td class=\"code\">");
        try!(out.write(&buffer));
        wf!(out, "</td></tr></table>\n");
        Ok(linecount)
    }
}

impl Formatter for HtmlFormatter {
    fn format<'a, I, W>(&mut self, source: I, out: W) -> Result<()>
        where I: Iterator<Item=Token<'a>>, W: Write
    {
        if self.nowrap {
            try!(self.format_lines(source, out));
        } else if self.linenos {
            try!(self.wrap_linenos(source, out));
        } else {
            try!(self.wrap_pre(source, out));
        }
        Ok(())
    }

    fn get_stylesheet(&self, mut arg: &str) -> String {
        let prefix = if arg.is_empty() {
            format!("pre.{}", self.pre_class)
        } else {
            arg.to_owned()
        };
        let mut result = String::new();
        // TODO
        result
    }
}
