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

//! Lexer for HTML.

define_lexer! {
    HtmlLexer,
    HTML_MACHINE,
}

define_machine! {
    HTML_MACHINE,
    HTML_TOKEN_DEF,
    [IGNORECASE, DOTALL],
    "root": [
        (r"[^<&]+", Text),
        (r"&[^\s;]*;", NameEntity),
        (r"<!\[CDATA\[.*?\]\]>", CommentPreproc),
        (r"<!--", Comment, "comment"),
        (r"<\?.*?\?>", CommentPreproc),
        (r"<![^>]*>", CommentPreproc),
        (r"(<)(\s*)(script)(\s*)", bygroups(Punctuation, Text, NameTag, Text),
         ("script-content", "tag")),
        (r"(<)(\s*)(style)(\s*)", bygroups(Punctuation, Text, NameTag, Text),
         ("style-content", "tag")),
        (r"(<)(\s*)([\w:.-]+)", bygroups(Punctuation, Text, NameTag), "tag"),
        (r"(<)(\s*)(/)(\s*)([\w:.-]+)(\s*)(>)",
         bygroups(Punctuation, Text, Punctuation, Text, NameTag, Text, Punctuation)),
    ],
    "comment": [
        (r"[^-]+", Comment),
        (r"-->", Comment, "#pop"),
        (r"-", Comment),
    ],
    "tag": [
        (r"\s+", Text),
        (r"([\w:-]+\s*)(=)(\s*)", bygroups(NameAttribute, Operator, Text), "attr"),
        (r"[\w:-]+", NameAttribute),
        (r"(/)(\s*)(>)", bygroups(Punctuation, Text, Punctuation), "#pop"),
        (r">", Punctuation, "#pop"),
    ],
    "attr": [
        (r#""[^"]*""#, String, "#pop"),
        (r"'[^']*'", String, "#pop"),
        (r"[^\s>]+", String, "#pop"),
    ],
    "script-content": [
        (r"[^<]+", Text),
        (r"(<)(\s*)(/)(\s*)(script)(\s*)(>)",
         bygroups(Punctuation, Text, Punctuation, Text, NameTag, Text, Punctuation), "#pop"),
        (r"<", Text),
    ],
    "style-content": [
        (r"[^<]+", Text),
        (r"(<)(\s*)(/)(\s*)(style)(\s*)(>)",
         bygroups(Punctuation, Text, Punctuation, Text, NameTag, Text, Punctuation), "#pop"),
        (r"<", Text),
    ],
}
