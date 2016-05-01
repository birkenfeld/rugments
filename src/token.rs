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

//! Definition of tokens and token types.

use std::fmt;

pub struct Token<'t> {
    pub text: &'t str,
    pub ttype: TokenType,
}

impl<'t> fmt::Debug for Token<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:-50} {:?}", format!("{:?}", self.text), self.ttype)
    }
}

/// Defines all allowable token types.  `Error`, `Other` and `Escape`
/// are special types that shouldn't be emitted from a lexer, but are
/// used by the system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Lexing error.
    Error,
    /// Text that doesn't belong to the used lexer.
    Other,
    /// Special: treated specially by formatters.
    Escape,

    /// General type for text.
    Text,
    /// Insignificant whitespace.
    Whitespace,
    /// Significant punctuation.
    Punctuation,
    /// Keywords.
    Keyword,
    KeywordConstant,
    KeywordDeclaration,
    KeywordNamespace,
    KeywordPseudo,
    KeywordReserved,
    KeywordType,
    /// Names.
    Name,
    NameAttribute,
    NameBuiltin,
    NameBuiltinPseudo,
    NameClass,
    NameConstant,
    NameDecorator,
    NameEntity,
    NameException,
    NameFunction,
    NameFunctionMagic,
    NameProperty,
    NameLabel,
    NameNamespace,
    NameOther,
    NameTag,
    NameVariable,
    NameVariableClass,
    NameVariableGlobal,
    NameVariableInstance,
    NameVariableMagic,
    /// String literals.
    String,
    StringAffix,
    StringBacktick,
    StringChar,
    StringDelimiter,
    StringDoc,
    StringDouble,
    StringEscape,
    StringHeredoc,
    StringInterpol,
    StringOther,
    StringRegex,
    StringSingle,
    StringSymbol,
    /// Number literals.
    Number,
    NumberBin,
    NumberFloat,
    NumberHex,
    NumberInteger,
    NumberIntegerLong,
    NumberOct,
    /// Other literals.
    Literal,
    LiteralDate,
    /// Operators (punctuation and words).
    Operator,
    OperatorWord,
    /// Comments.
    Comment,
    CommentHashbang,
    CommentMultiline,
    CommentPreproc,
    CommentPreprocFile,
    CommentSingle,
    CommentSpecial,

    /// Generic types.
    Generic,
    GenericDeleted,
    GenericEmph,
    GenericError,
    GenericHeading,
    GenericInserted,
    GenericOutput,
    GenericPrompt,
    GenericStrong,
    GenericSubheading,
    GenericTraceback,

    /// Custom type.
    Custom(&'static str),
}
pub use self::TokenType::*;

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Error => "Error",
            Other => "Other",
            Escape => "Escape",
            Text => "Text",
            Whitespace => "Whitespace",
            Punctuation => "Punctuation",
            Keyword => "Keyword",
            KeywordConstant => "Keyword.Constant",
            KeywordDeclaration => "Keyword.Declaration",
            KeywordNamespace => "Keyword.Namespace",
            KeywordPseudo => "Keyword.Pseudo",
            KeywordReserved => "Keyword.Reserved",
            KeywordType => "Keyword.Type",
            Name => "Name",
            NameAttribute => "Name.Attribute",
            NameBuiltin => "Name.Builtin",
            NameBuiltinPseudo => "Name.Builtin.Pseudo",
            NameClass => "Name.Class",
            NameConstant => "Name.Constant",
            NameDecorator => "Name.Decorator",
            NameEntity => "Name.Entity",
            NameException => "Name.Exception",
            NameFunction => "Name.Function",
            NameFunctionMagic => "Name.Function.Magic",
            NameProperty => "Name.Property",
            NameLabel => "Name.Label",
            NameNamespace => "Name.Namespace",
            NameOther => "Name.Other",
            NameTag => "Name.Tag",
            NameVariable => "Name.Variable",
            NameVariableClass => "Name.Variable.Class",
            NameVariableGlobal => "Name.Variable.Global",
            NameVariableInstance => "Name.Variable.Instance",
            NameVariableMagic => "Name.Variable.Magic",
            String => "String",
            StringAffix => "String.Affix",
            StringBacktick => "String.Backtick",
            StringChar => "String.Char",
            StringDelimiter => "String.Delimiter",
            StringDoc => "String.Doc",
            StringDouble => "String.Double",
            StringEscape => "String.Escape",
            StringHeredoc => "String.Heredoc",
            StringInterpol => "String.Interpol",
            StringOther => "String.Other",
            StringRegex => "String.Regex",
            StringSingle => "String.Single",
            StringSymbol => "String.Symbol",
            Number => "Number",
            NumberBin => "Number.Bin",
            NumberFloat => "Number.Float",
            NumberHex => "Number.Hex",
            NumberInteger => "Number.Integer",
            NumberIntegerLong => "Number.Integer.Long",
            NumberOct => "Number.Oct",
            Literal => "Literal",
            LiteralDate => "Literal.Date",
            Operator => "Operator",
            OperatorWord => "Operator.Word",
            Comment => "Comment",
            CommentHashbang => "Comment.Hashbang",
            CommentMultiline => "Comment.Multiline",
            CommentPreproc => "Comment.Preproc",
            CommentPreprocFile => "Comment.Preproc.File",
            CommentSingle => "Comment.Single",
            CommentSpecial => "Comment.Special",
            Generic => "Generic",
            GenericDeleted => "Generic.Deleted",
            GenericEmph => "Generic.Emph",
            GenericError => "Generic.Error",
            GenericHeading => "Generic.Heading",
            GenericInserted => "Generic.Inserted",
            GenericOutput => "Generic.Output",
            GenericPrompt => "Generic.Prompt",
            GenericStrong => "Generic.Strong",
            GenericSubheading => "Generic.Subheading",
            GenericTraceback => "Generic.Traceback",
            Custom(s) => s,
        }
    }

    pub fn as_short_str(&self) -> &'static str {
        match *self {
            Error => "err",
            Other => "x",
            Escape => "esc",
            Text => "",
            Whitespace => "w",
            Punctuation => "p",
            Keyword => "k",
            KeywordConstant => "kc",
            KeywordDeclaration => "kd",
            KeywordNamespace => "kn",
            KeywordPseudo => "kp",
            KeywordReserved => "kr",
            KeywordType => "kt",
            Name => "n",
            NameAttribute => "na",
            NameBuiltin => "nb",
            NameBuiltinPseudo => "np",
            NameClass => "nc",
            NameConstant => "no",
            NameDecorator => "nd",
            NameEntity => "ni",
            NameException => "ne",
            NameFunction => "nf",
            NameFunctionMagic => "fm",
            NameProperty => "py",
            NameLabel => "nl",
            NameNamespace => "nn",
            NameOther => "nx",
            NameTag => "nt",
            NameVariable => "nv",
            NameVariableClass => "vc",
            NameVariableGlobal => "vg",
            NameVariableInstance => "vi",
            NameVariableMagic => "vm",
            String => "s",
            StringAffix => "s",
            StringBacktick => "sb",
            StringChar => "sc",
            StringDelimiter => "dl",
            StringDoc => "sd",
            StringDouble => "s2",
            StringEscape => "se",
            StringHeredoc => "sh",
            StringInterpol => "si",
            StringOther => "sx",
            StringRegex => "sr",
            StringSingle => "s1",
            StringSymbol => "ss",
            Number => "m",
            NumberBin => "mb",
            NumberFloat => "mf",
            NumberHex => "mh",
            NumberInteger => "mi",
            NumberIntegerLong => "il",
            NumberOct => "mo",
            Literal => "l",
            LiteralDate => "ld",
            Operator => "o",
            OperatorWord => "ow",
            Comment => "c",
            CommentHashbang => "ch",
            CommentMultiline => "cm",
            CommentPreproc => "cp",
            CommentPreprocFile => "cpf",
            CommentSingle => "c1",
            CommentSpecial => "cs",
            Generic => "g",
            GenericDeleted => "gd",
            GenericEmph => "ge",
            GenericError => "gr",
            GenericHeading => "gh",
            GenericInserted => "gi",
            GenericOutput => "go",
            GenericPrompt => "gp",
            GenericStrong => "gs",
            GenericSubheading => "gu",
            GenericTraceback => "gt",
            // TODO: this needs new static strings
            Custom(s) => s,
        }
    }

    pub fn from_str(s: &'static str) -> Self {
        match s {
            "Error" => Error,
            "Other" => Other,
            "Escape" => Escape,
            "Text" => Text,
            "Whitespace" => Whitespace,
            "Punctuation" => Punctuation,
            "Keyword" => Keyword,
            "Keyword.Constant" => KeywordConstant,
            "Keyword.Declaration" => KeywordDeclaration,
            "Keyword.Namespace" => KeywordNamespace,
            "Keyword.Pseudo" => KeywordPseudo,
            "Keyword.Reserved" => KeywordReserved,
            "Keyword.Type" => KeywordType,
            "Name" => Name,
            "Name.Attribute" => NameAttribute,
            "Name.Builtin" => NameBuiltin,
            "Name.Builtin.Pseudo" => NameBuiltinPseudo,
            "Name.Class" => NameClass,
            "Name.Constant" => NameConstant,
            "Name.Decorator" => NameDecorator,
            "Name.Entity" => NameEntity,
            "Name.Exception" => NameException,
            "Name.Function" => NameFunction,
            "Name.Function.Magic" => NameFunctionMagic,
            "Name.Property" => NameProperty,
            "Name.Label" => NameLabel,
            "Name.Namespace" => NameNamespace,
            "Name.Other" => NameOther,
            "Name.Tag" => NameTag,
            "Name.Variable" => NameVariable,
            "Name.Variable.Class" => NameVariableClass,
            "Name.Variable.Global" => NameVariableGlobal,
            "Name.Variable.Instance" => NameVariableInstance,
            "Name.Variable.Magic" => NameVariableMagic,
            "String" => String,
            "String.Affix" => StringAffix,
            "String.Backtick" => StringBacktick,
            "String.Char" => StringChar,
            "String.Delimiter" => StringDelimiter,
            "String.Doc" => StringDoc,
            "String.Double" => StringDouble,
            "String.Escape" => StringEscape,
            "String.Heredoc" => StringHeredoc,
            "String.Interpol" => StringInterpol,
            "String.Other" => StringOther,
            "String.Regex" => StringRegex,
            "String.Single" => StringSingle,
            "String.Symbol" => StringSymbol,
            "Number" => Number,
            "Number.Bin" => NumberBin,
            "Number.Float" => NumberFloat,
            "Number.Hex" => NumberHex,
            "Number.Integer" => NumberInteger,
            "Number.Integer.Long" => NumberIntegerLong,
            "Number.Oct" => NumberOct,
            "Literal" => Literal,
            "Literal.Date" => LiteralDate,
            "Operator" => Operator,
            "Operator.Word" => OperatorWord,
            "Comment" => Comment,
            "Comment.Hashbang" => CommentHashbang,
            "Comment.Multiline" => CommentMultiline,
            "Comment.Preproc" => CommentPreproc,
            "Comment.Preproc.File" => CommentPreprocFile,
            "Comment.Single" => CommentSingle,
            "Comment.Special" => CommentSpecial,
            "Generic" => Generic,
            "Generic.Deleted" => GenericDeleted,
            "Generic.Emph" => GenericEmph,
            "Generic.Error" => GenericError,
            "Generic.Heading" => GenericHeading,
            "Generic.Inserted" => GenericInserted,
            "Generic.Output" => GenericOutput,
            "Generic.Prompt" => GenericPrompt,
            "Generic.Strong" => GenericStrong,
            "Generic.Subheading" => GenericSubheading,
            "Generic.Traceback" => GenericTraceback,
            s => Custom(s),
        }
    }
}
