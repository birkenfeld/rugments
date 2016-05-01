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

use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

use token::{Token, TokenType};
use macros::{MachineDef, RuleDef};

pub trait Lexer<'t>: Iterator<Item=Token<'t>> {
    // Currently unclear which other methods belong here.
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule(Regex, MatchAction, StateAction);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchAction {
    Single(TokenType),
    ByGroups(&'static [TokenType]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateAction {
    None,
    Pop,
    PopMulti(usize),
    Push(&'static str),
    PushMulti(&'static [&'static str]),
    PushSelf(usize),
}

pub struct State(Vec<Rule>);

pub struct Machine(BTreeMap<&'static str, State>);

impl Machine {
    pub fn get_state<'a>(&'a self, state: &str) -> &'a State {
        &self.0[state]
    }

    pub fn convert(machine: MachineDef) -> Machine {
        let mut map = BTreeMap::new();
        for &(statename, statedef) in machine {
            let mut rules = Vec::new();
            for ruledef in statedef {
                // TODO: CHECK stuff!
                match *ruledef {
                    RuleDef::Regex(rx, maction, saction) => {
                        rules.push(Rule(Regex::new(rx).unwrap(), maction, saction));
                    }
                    // Words(words, maction, saction) => {
                    //     // XXX
                    // }
                    RuleDef::Default(saction) => {
                        rules.push(Rule(Regex::new("").unwrap(),
                                        MatchAction::Single(TokenType::Text), saction));
                    }
                    _ => {}
                }
            }
            map.insert(statename, State(rules));
        }
        Machine(map)
    }
}

pub struct RegexLexer<'t> {
    machine: &'static Machine,
    states: Vec<&'static str>,
    topstate: &'static State,
    queue: VecDeque<Token<'t>>,
    rest: &'t str,
}

impl<'t> RegexLexer<'t> {
    pub fn new(machine: &'static Machine, initstate: &'static str, text: &'t str)
               -> RegexLexer<'t> {
        RegexLexer { machine: machine,
                     states: vec![initstate],
                     topstate: machine.get_state(initstate),
                     queue: VecDeque::with_capacity(16),
                     rest: text }
    }

    #[inline]
    fn do_state_action(&mut self, action: StateAction) {
        match action {
            StateAction::None => { }
            StateAction::Pop => {
                self.states.pop();
                self.topstate = &self.machine.get_state(self.states.last().unwrap());
            }
            StateAction::PopMulti(n) => {
                for _ in 0..n { self.states.pop(); }
                self.topstate = &self.machine.get_state(self.states.last().unwrap());
            }
            StateAction::PushSelf(n) => {
                let cur = self.states.last().unwrap().clone();
                for _ in 0..n { self.states.push(cur); }
            }
            StateAction::Push(to) => {
                self.states.push(to);
                self.topstate = &self.machine.get_state(to);
            }
            StateAction::PushMulti(which) => {
                self.states.extend(which);
                self.topstate = &self.machine.get_state(self.states.last().unwrap());
            }
        }
    }

    #[inline]
    fn lex_next(&mut self) -> Option<Token<'t>> {
        for &Rule(ref rx, type_action, state_action) in &self.topstate.0 {
            match type_action {
                MatchAction::Single(ttype) => if let Some((_, idx)) = rx.find(self.rest) {
                    let (matched, rest) = self.rest.split_at(idx);
                    self.rest = rest;
                    self.do_state_action(state_action);
                    return Some(Token { text: matched, ttype: ttype });
                },
                MatchAction::ByGroups(groups) => if let Some(cap) = rx.captures(self.rest) {
                    self.rest = &self.rest[cap.pos(0).unwrap().1..];
                    self.do_state_action(state_action);
                    let mut first = None;
                    for (i, &group_ttype) in groups.iter().enumerate() {
                        let matched = cap.at(i + 1).unwrap();
                        let tok = Token { text: matched, ttype: group_ttype };
                        if i == 0 {
                            first = Some(tok);
                        } else if !matched.is_empty() {
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
        Some(Token { text: matched, ttype: TokenType::Error })
    }
}

impl<'t> Iterator for RegexLexer<'t> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.queue.pop_back() {
            Some(v)
        } else {
            self.lex_next()
        }
    }
}
