#![allow(dead_code)]

use crate::automaton::dfa::{DFADesign, DFARulebook};
use crate::automaton::farule::{FARule, State, TransitionType};
use std::char;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

#[derive(Debug)]
pub struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    // 全ての文字列にマッチすることを示すchar
    pub const EVERYTHING_CHAR: char = '∀';

    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook { rules }
    }

    // ルールに適用されている入力文字の配列を取得
    pub fn alphabet(&self) -> HashSet<char> {
        self.rules
            .iter()
            .filter(|r| {
                matches!(
                    r.transition,
                    TransitionType::Character(_) | TransitionType::Everything
                )
            })
            .map(|r| match r.transition {
                TransitionType::Character(c) => c,
                TransitionType::Everything => Self::EVERYTHING_CHAR,
                _ => panic!("[NFARulebook::alphabet] must not reach"),
            })
            .collect()
    }

    // 現在の状態から次の状態の集合を取得
    pub fn next_state(&self, states: &HashSet<State>, c: Option<char>) -> HashSet<State> {
        let mut next_states = vec![];
        states.iter().for_each(|s| {
            let mut t = self.rule_for(s, c);
            next_states.append(&mut t);
        });

        next_states.into_iter().collect::<HashSet<State>>()
    }

    // ルールを適用し、次の状態を返す
    fn rule_for(&self, s: &State, c: Option<char>) -> Vec<State> {
        self.rules
            .iter()
            .filter(|r| r.applies_to(s, &c))
            .map(|r| *r.follow())
            .collect::<Vec<State>>()
    }
}

struct Nfa<'a> {
    current_state: HashSet<State>,
    accept_states: &'a [State],
    rulebook: &'a NFARulebook,
}

impl<'a> Nfa<'a> {
    pub fn new(
        current_state: HashSet<State>,
        accept_states: &'a [State],
        rulebook: &'a NFARulebook,
    ) -> Self {
        Nfa {
            current_state,
            accept_states,
            rulebook,
        }
    }

    pub fn current_state(&self) -> HashSet<State> {
        self.current_state.clone()
    }

    pub fn accepting(&self) -> bool {
        self.current_state
            .iter()
            .any(|c| self.accept_states.iter().any(|a| a == c))
    }

    pub fn read_string(&mut self, s: &str) {
        s.chars().for_each(|c| {
            // ε遷移を行ってから通常遷移
            self.trans_epsilon();
            self.current_state = self.rulebook.next_state(&self.current_state, Some(c));
        });

        // 読み込み完了後、ε遷移
        self.trans_epsilon();
    }

    fn trans_epsilon(&mut self) {
        // ε遷移の結果がサブセットにならなくなるまで遷移
        let epsilon = self.rulebook.next_state(&self.current_state, None);
        if epsilon.is_subset(&self.current_state) {
            return;
        }

        self.current_state.extend(epsilon);
        self.trans_epsilon();
    }
}

#[derive(Debug)]
pub struct NFADesign<'a> {
    start_state: State,
    accept_states: &'a [State],
    rulebook: &'a NFARulebook,
}

impl<'a> NFADesign<'a> {
    pub fn new(start_state: State, accept_states: &'a [State], rulebook: &'a NFARulebook) -> Self {
        NFADesign {
            start_state,
            accept_states,
            rulebook,
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut start_state = HashSet::new();
        start_state.insert(self.start_state);

        let mut nfa = Nfa::new(start_state, self.accept_states, self.rulebook);
        nfa.read_string(s);
        nfa.accepting()
    }
}

#[derive(Debug)]
struct StateMap {
    start: HashSet<State>,
    map: HashMap<Vec<State>, State>,
}

impl StateMap {
    pub fn new() -> Self {
        StateMap {
            start: HashSet::new(),
            map: HashMap::new(),
        }
    }

    // 開始集合登録
    pub fn insert_start(&mut self, start: State) {
        self.start.insert(start);
    }

    // 開始集合に対応するState取得
    pub fn get_start(&self) -> State {
        *self.map.get(&self.key(&self.start)).unwrap_or_else(|| {
            panic!(
                "[StateMap::get_start] get start is error (Start: {:?})",
                self.start
            )
        })
    }

    // 指定された状態に合致する状態を取得
    pub fn get_state(&self, state: &HashSet<State>) -> State {
        *self.map.get(&self.key(state)).unwrap_or_else(|| {
            panic!(
                "[StateMap::get_state] get state is error (State: {:?})",
                state
            )
        })
    }

    // 指定された状態が集合に含まれているものを取得
    pub fn get_include_state(&self, state: &State) -> Vec<State> {
        let mut states = vec![];
        for (k, v) in self.map.iter() {
            if k.contains(state) {
                states.push(*v);
            }
        }

        states
    }

    // すでに集合が登録されているか
    pub fn is_inserted(&self, set: &HashSet<State>) -> bool {
        self.map.contains_key(&self.key(set))
    }

    // Mapへ登録
    pub fn insert_map(&mut self, set: &HashSet<State>) {
        self.map.insert(self.key(set), State::create_at_rnd());
    }

    // キー生成
    fn key(&self, key: &HashSet<State>) -> Vec<State> {
        // ソートしないと並び順が異なり、キーが一致しない
        let mut k: Vec<State> = key.clone().into_iter().collect();
        k.sort();

        k
    }
}

#[derive(Debug)]
pub struct NFAConverter<'a> {
    start_state: State,
    accept_states: &'a [State],
    rulebook: &'a NFARulebook,
    state_map: StateMap,
    dfa_rulebook: Option<DFARulebook>,
}

type DtranRule = (HashSet<State>, char, HashSet<State>);
impl<'a> NFAConverter<'a> {
    pub fn new(start: State, accept_states: &'a [State], rulebook: &'a NFARulebook) -> Self {
        NFAConverter {
            start_state: start,
            accept_states,
            rulebook,
            state_map: StateMap::new(),
            dfa_rulebook: None,
        }
    }

    // DFAルールの作成
    pub fn build(&mut self) -> &Self {
        // DFAルール作成
        let mut st_set = HashSet::new();
        st_set.insert(self.start_state);
        self.dfa_rulebook = Some(self.dfa_rulebook(&st_set));

        self
    }

    // NFA→DFA変換後の受理状態を返す
    pub fn accept(&self, str: &str) -> bool {
        // DFAを生成し、マッチ実行
        match &self.dfa_rulebook {
            Some(rulebook) => {
                // マップされた情報からスタートと受理状態を抽出
                let dfa_start = self.state_map.get_start();
                let dfa_accept = self.to_dfa_accept();
                DFADesign::new(dfa_start, &dfa_accept, rulebook).accept(str)
            }
            _ => false,
        }
    }

    // DFAルール作成
    fn dfa_rulebook(&mut self, start: &HashSet<State>) -> DFARulebook {
        let mut dtran = vec![];

        // ε遷移を行い、各入力文字に対する遷移を行う
        let mut queue = VecDeque::new();
        let mut searched_set = vec![];
        let ep_start = self.epsilon(start);
        queue.push_back(ep_start.clone());

        // イプシロン遷移後の状態を開始状態として登録
        self.save_start_state(&ep_start);

        while !queue.is_empty() {
            // 探索済み配列へ追加
            let set = queue.pop_front().unwrap_or_else(|| {
                panic!(
                    "[NFAConverter::expand_epsilon] pop_front is error (queue: {:?})",
                    queue
                )
            });
            searched_set.push(set.clone());

            // 各文字から遷移する集合を取得
            self.rulebook.alphabet().into_iter().for_each(|c| {
                let next_set = self.rulebook.next_state(&set, Some(c));
                let ep_next_set = self.epsilon(&next_set);

                // 遷移後の状態を探索していなければ、探索配列へ登録
                dtran.push((set.clone(), c, ep_next_set.clone()));
                if !searched_set.contains(&ep_next_set) {
                    queue.push_back(ep_next_set);
                }
            })
        }

        DFARulebook::new(self.farule(dtran))
    }

    // 部分集合構成法により抽出したルールをFARuleへ
    fn farule(&mut self, dtran: Vec<DtranRule>) -> Vec<FARule> {
        dtran
            .into_iter()
            .map(|d| {
                // 各状態の集合に対応する状態を作成しながら、ルールを作成
                let (st_state, character, next_state) = d;

                // 集合に対する状態が登録されていなければ、登録
                if !self.state_map.is_inserted(&st_state) {
                    self.state_map.insert_map(&st_state);
                }
                if !self.state_map.is_inserted(&next_state) {
                    self.state_map.insert_map(&next_state);
                }

                // FARuleを作成
                self.build_farule(character, &st_state, &next_state)
            })
            .collect()
    }

    // ε遷移
    fn epsilon(&self, start: &HashSet<State>) -> HashSet<State> {
        let mut s = start.clone();
        let ep = self.rulebook.next_state(&s, None);
        if ep.is_subset(&s) {
            // ε遷移の集合を返す
            return s;
        }

        // 遷移後の集合をマージ
        s.extend(ep);
        self.epsilon(&s)
    }

    // 探索済み判定
    fn is_searched(&self, searched_set: &[HashSet<State>], set: &HashSet<State>) -> bool {
        searched_set.iter().any(|s| s == set)
    }

    // 与えられた状態から遷移可能な集合を返す
    fn next_state(&self, state: HashSet<State>, s: &str) -> HashSet<State> {
        let accept_state = vec![];
        let mut nfa = Nfa::new(state, &accept_state, self.rulebook);
        nfa.read_string(s);
        nfa.current_state()
    }

    // DFA開始状態を登録
    fn save_start_state(&mut self, start: &HashSet<State>) {
        start
            .clone()
            .into_iter()
            .for_each(|s| self.state_map.insert_start(s));
    }

    // DFA受理状態を取得
    fn to_dfa_accept(&self) -> Vec<State> {
        self.accept_states
            .iter()
            .map(|s| self.state_map.get_include_state(s))
            .flatten()
            .collect()
    }

    // FARule作成
    fn build_farule(
        &self,
        c: char,
        st_state: &HashSet<State>,
        next_state: &HashSet<State>,
    ) -> FARule {
        if c == NFARulebook::EVERYTHING_CHAR {
            FARule::new(
                self.state_map.get_state(st_state),
                TransitionType::Everything,
                self.state_map.get_state(next_state),
            )
        } else {
            FARule::new(
                self.state_map.get_state(st_state),
                TransitionType::Character(c),
                self.state_map.get_state(next_state),
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::farule::TransitionType;

    #[test]
    fn test_nfarulebook() {
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
                FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
            ]);

            assert_eq!(
                vec![State::new(1), State::new(2)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('b'))
            );
            assert_eq!(
                vec![State::new(1), State::new(2), State::new(3)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(
                    &vec![State::new(1), State::new(2)].into_iter().collect(),
                    Some('b')
                )
            );
            assert_eq!(
                vec![State::new(1), State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(
                    &vec![State::new(1), State::new(3)].into_iter().collect(),
                    Some('b')
                )
            );
        }
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            ]);

            assert_eq!(
                vec![State::new(2)].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), None)
            );
            assert_eq!(
                vec![State::new(2)].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('a'))
            );
            assert_eq!(
                vec![].into_iter().collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), Some('b'))
            );
        }
        {
            let book = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(4)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(2)),
                FARule::new(State::new(4), TransitionType::Character('a'), State::new(5)),
                FARule::new(State::new(5), TransitionType::Character('a'), State::new(6)),
                FARule::new(State::new(6), TransitionType::Character('a'), State::new(4)),
            ]);

            assert_eq!(
                vec![State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                book.next_state(&vec![State::new(1)].into_iter().collect(), None)
            );
        }
    }

    #[test]
    fn test_nfarulebook_alphabet() {
        let rule = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(3), TransitionType::Epsilon, State::new(2)),
        ]);

        assert_eq!(
            vec!['a', 'b'].into_iter().collect::<HashSet<char>>(),
            rule.alphabet()
        );
    }

    #[test]
    fn test_nfa_accepting() {
        let book = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
        ]);

        assert_eq!(
            false,
            Nfa::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &vec![State::new(4)],
                &book
            )
            .accepting()
        );
        assert_eq!(
            true,
            Nfa::new(
                vec![State::new(1), State::new(2), State::new(4)]
                    .into_iter()
                    .collect::<HashSet<State>>(),
                &vec![State::new(4)],
                &book
            )
            .accepting()
        );
    }

    #[test]
    fn test_nfa_read_string() {
        let book = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
        ]);

        {
            let accept_states = vec![State::new(4)];
            let mut nfa = Nfa::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &accept_states,
                &book,
            );
            nfa.read_string("bab");

            assert!(nfa.accepting());
        }
        {
            let accept_states = vec![State::new(4)];
            let mut nfa = Nfa::new(
                vec![State::new(1)].into_iter().collect::<HashSet<State>>(),
                &accept_states,
                &book,
            );
            nfa.read_string("bbbbb");

            assert!(nfa.accepting());
        }
    }

    #[test]
    fn test_nfa_design() {
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
                FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(4)];
            let design = NFADesign::new(State::new(1), &accept_statuses, &rule);

            assert!(design.accept("bab"));
            assert!(design.accept("bbbbb"));
            assert!(!design.accept("bbabb"));
        }
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(4)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(2)),
                FARule::new(State::new(4), TransitionType::Character('a'), State::new(5)),
                FARule::new(State::new(5), TransitionType::Character('a'), State::new(6)),
                FARule::new(State::new(6), TransitionType::Character('a'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(2), State::new(4)];
            let design = NFADesign::new(State::new(1), &accept_statuses, &rule);

            assert!(!design.accept("a"));
            assert!(design.accept("aa"));
            assert!(design.accept("aaa"));
            assert!(design.accept("aaaa"));
            assert!(!design.accept("aaaaa"));
            assert!(design.accept("aaaaaa"));
            assert!(design.accept("aaaaaa"));
        }
    }

    #[test]
    fn test_nfa_converter_next_state() {
        let rule = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(3), TransitionType::Epsilon, State::new(2)),
        ]);

        let accept_statuses = vec![];
        let converter = NFAConverter::new(State::new(1), &accept_statuses, &rule);

        assert_eq!(
            vec![State::new(1), State::new(2)]
                .into_iter()
                .collect::<HashSet<State>>(),
            converter.next_state(vec![State::new(1)].into_iter().collect(), "a")
        );
        assert_eq!(
            vec![State::new(3), State::new(2)]
                .into_iter()
                .collect::<HashSet<State>>(),
            converter.next_state(vec![State::new(2)].into_iter().collect(), "b")
        );
        assert_eq!(
            vec![State::new(1), State::new(2), State::new(3)]
                .into_iter()
                .collect::<HashSet<State>>(),
            converter.next_state(vec![State::new(3)].into_iter().collect(), "b")
        );
    }

    #[test]
    fn test_nfa_converter_epsilon() {
        let rule = NFARulebook::new(vec![
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
            FARule::new(State::new(1), TransitionType::Character('a'), State::new(2)),
            FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
            FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
            FARule::new(State::new(2), TransitionType::Epsilon, State::new(4)),
            FARule::new(State::new(3), TransitionType::Character('b'), State::new(1)),
            FARule::new(State::new(3), TransitionType::Epsilon, State::new(2)),
        ]);

        let accept_statuses = vec![];
        let converter = NFAConverter::new(State::new(1), &accept_statuses, &rule);

        assert_eq!(
            vec![State::new(1), State::new(2), State::new(4)]
                .into_iter()
                .collect::<HashSet<State>>(),
            converter.epsilon(&vec![State::new(1)].into_iter().collect())
        );
        assert_eq!(
            vec![State::new(2), State::new(3), State::new(4)]
                .into_iter()
                .collect::<HashSet<State>>(),
            converter.epsilon(&vec![State::new(3)].into_iter().collect())
        );
    }

    #[test]
    fn test_nfa_converter_accept() {
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Character('a'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(1)),
                FARule::new(State::new(1), TransitionType::Character('b'), State::new(2)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(2), TransitionType::Character('b'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(4)),
                FARule::new(State::new(3), TransitionType::Character('b'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(4)];
            let mut converter = NFAConverter::new(State::new(1), &accept_statuses, &rule);
            let dfa = converter.build();

            assert!(dfa.accept("bab"));
            assert!(dfa.accept("bbbbb"));
            assert!(!dfa.accept("bbabb"));
        }
        {
            let rule = NFARulebook::new(vec![
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(2)),
                FARule::new(State::new(1), TransitionType::Epsilon, State::new(4)),
                FARule::new(State::new(2), TransitionType::Character('a'), State::new(3)),
                FARule::new(State::new(3), TransitionType::Character('a'), State::new(2)),
                FARule::new(State::new(4), TransitionType::Character('a'), State::new(5)),
                FARule::new(State::new(5), TransitionType::Character('a'), State::new(6)),
                FARule::new(State::new(6), TransitionType::Character('a'), State::new(4)),
            ]);

            let accept_statuses = vec![State::new(2), State::new(4)];
            let mut converter = NFAConverter::new(State::new(1), &accept_statuses, &rule);
            let dfa = converter.build();

            assert!(!dfa.accept("a"));
            assert!(dfa.accept("aa"));
            assert!(dfa.accept("aaa"));
            assert!(dfa.accept("aaaa"));
            assert!(!dfa.accept("aaaaa"));
            assert!(dfa.accept("aaaaaa"));
            assert!(dfa.accept("aaaaaa"));
        }
    }
}
