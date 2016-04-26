// Copyright (c) 2015-2016 Georg Brandl.  Licensed under the Apache License,
// Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed except
// according to those terms.

use regex::Regex;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    Name,
    Comment,
    String,
    Punctuation,
    Error,
}
use self::TokenType::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeAction {
    T(TokenType),
    ByGroups(&'static [TokenType]),
}
use self::TypeAction::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateAction {
    No,
    Pop,
    PopMulti(usize),
    Push(&'static str),
    PushMulti(&'static [&'static str]),
    PushSelf(usize),
}
use self::StateAction::*;

pub struct Token<'t> {
    text: &'t str,
    ttype: TokenType
}

impl<'t> fmt::Debug for Token<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:-50} {:?}", format!("{:?}", self.text), self.ttype)
    }
}

pub type RuleDef = (&'static str, TypeAction, StateAction);
pub type StateDef = (&'static str, &'static [RuleDef]);
pub type MachineDef = &'static [StateDef];

pub type Rule = (Regex, TypeAction, StateAction);
pub type State = Vec<Rule>;
pub type Machine = BTreeMap<&'static str, State>;

pub fn to_machine(machine: MachineDef) -> Machine {
    let mut map = BTreeMap::new();
    for &(statename, statedef) in machine {
        let mut rules = Vec::new();
        for ruledef in statedef {
            // CHECK stuff!
            rules.push((Regex::new(ruledef.0).unwrap(),
                        ruledef.1, ruledef.2));
        }
        map.insert(statename, rules);
    }
    map
}

const HTML_TOKENS: MachineDef = &[
    ("root", &[
        (r"\A[^<&]+", T(Text), No),
        (r"\A&\S*?;", T(Name), No),
        (r"\A<!\[CDATA\[.*?\]\]>", T(Comment), No),
        (r"\A<!--", T(Comment), Push("comment")),
        (r"\A<![^>]*>", T(Comment), No),
        (r"\A(<)(\s*)(script)(\s*)",
         ByGroups(&[Punctuation, Text, Name, Text]),
         PushMulti(&["script-content", "tag"])),
        (r"\A(<)(\s*)(style)(\s*)",
         ByGroups(&[Punctuation, Text, Name, Text]),
         PushMulti(&["style-content", "tag"])),
        (r"\A(<)(\s*)([\w:.-]+)",
         ByGroups(&[Punctuation, Text, Name]),
         Push("tag")),
        (r"\A(<)(\s*)(/)(\s*)([\w:.-]+)(\s*)(>)",
         ByGroups(&[Punctuation, Text, Punctuation, Text, Name, Text, Punctuation]),
         No),
    ]),
    ("comment", &[
        (r"\A[^-]+", T(Comment), No),
        (r"\A-->", T(Comment), Pop),
        (r"\A-", T(Comment), No),
    ]),
    ("tag", &[
        (r"\A\s+", T(Text), No),
        (r"\A([\w:-]+\s*)(=)(\s*)",
         T(Name), Push("attr")), //  bygroups(Name.Attribute, Operator, Text),
        (r"\A[\w:-]+", T(Name), No),
        (r"\A(/?)(\s*)(>)",
         T(Name), // bygroups(Punctuation, Text, Punctuation),
         Pop),
    ]),
    ("attr", &[
        ("\\A\".*?\"", T(String), Pop),
        (r"\A'.*?'", T(String), Pop),
        (r"\A[^\s>]+", T(String), Pop),
    ]),
    ("script-content", &[
        (r"\A(<)(\s*)(/)(\s*)(script)(\s*)(>)",
         T(Name),
         // bygroups(Punctuation, Text, Punctuation, Text, Name.Tag, Text, Punctuation),
         Pop),
        (r"(?s)\A.+?(<\s*/\s*script\s*>)", T(Text), Pop),
    ]),
    ("style-content", &[
        (r"\A(<)(\s*)(/)(\s*)(style)(\s*)(>)",
         T(Name), // bygroups(Punctuation, Text, Punctuation, Text, Name.Tag, Text, Punctuation),
         Pop),
        (r"(?s)\A.+?(<\s*/\s*style\s*>)", T(Text), Pop),
    ]),
];

lazy_static! {
    pub static ref HTML_MACHINE: Machine = to_machine(HTML_TOKENS);
}

pub struct HtmlLexer<'t> {
    machine: &'static Machine,
    states: Vec<&'static str>,
    topstate: &'static State,
    queue: VecDeque<Token<'t>>,
    rest: &'t str,
}

impl<'t> HtmlLexer<'t> {
    pub fn new(text: &'t str) -> HtmlLexer<'t> {
        HtmlLexer { machine: &HTML_MACHINE,
                    states: vec!["root"],
                    topstate: &HTML_MACHINE["root"],
                    queue: VecDeque::with_capacity(16),
                    rest: text }
    }

    #[inline]
    fn do_state_action(&mut self, action: StateAction) {
        match action {
            No => (),
            Pop => {
                self.states.pop();
                self.topstate = &self.machine[self.states.last().unwrap()];
            }
            PopMulti(n) => {
                for _ in 0..n { self.states.pop(); }
                self.topstate = &self.machine[self.states.last().unwrap()];
            }
            PushSelf(n) => {
                let cur = self.states.last().unwrap().clone();
                for _ in 0..n { self.states.push(cur); }
            }
            Push(to) => {
                self.states.push(to);
                self.topstate = &self.machine[to];
            }
            PushMulti(which) => {
                self.states.extend(which);
                self.topstate = &self.machine[self.states.last().unwrap()];
            }
        }
    }
}

impl<'t> Iterator for HtmlLexer<'t> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            return self.queue.pop_back();
        }
        for &(ref rx, type_action, state_action) in self.topstate {
            match type_action {
                T(tt) => if let Some((_, idx)) = rx.find(self.rest) {
                    self.do_state_action(state_action);
                    let (matched, rest) = self.rest.split_at(idx);
                    self.rest = rest;
                    return Some(Token { text: matched, ttype: tt });
                },
                ByGroups(groups) => if let Some(cap) = rx.captures(&self.rest) {
                    self.rest = &self.rest[cap.pos(0).unwrap().1..];
                    self.do_state_action(state_action);
                    let mut first = None;
                    for (i, grtt) in groups.iter().enumerate() {
                        let tok = Token { text: cap.at(i + 1).unwrap(), ttype: *grtt };
                        if i == 0 {
                            first = Some(tok);
                        } else {
                            self.queue.push_front(tok);
                        }
                    }
                    return first;
                }
            }
        }
        if self.rest.is_empty() {
            return None;
        }
        let idx = self.rest.char_indices().skip(1).next().map_or(self.rest.len(), |v| v.0);
        let (matched, rest) = self.rest.split_at(idx);
        self.rest = rest;
        Some(Token { text: matched, ttype: Error })
    }
}
