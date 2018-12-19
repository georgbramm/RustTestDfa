use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

#[derive(Debug)]
pub struct Automaton<T>
    where T: Hash + Eq {
    states: HashMap<State, HashMap<T, State>>,
    start: State,
    current: State,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct State {
    id: usize,
    end: bool
}

impl<T> Automaton<T>
    where T: Hash + Eq {

    pub fn new(start: State) -> Self {
        let mut r = Automaton {
            states: HashMap::new(),
            start: start,
            current: start,
        };
        r.add_state(start);
        r
    }

    pub fn add_state(&mut self, state: State) {
        self.states.insert(state, HashMap::new());
    }

    pub fn add_transition(&mut self, init: State, end: State, val: T) {
        if let Some(t) = self.states.get_mut(&init) {
            t.insert(val, end);
        }
    }

    pub fn consume(&mut self, val: T) {
        let t = self.states.get(&self.current).unwrap();
        if let Some(s) = t.get(&val) {
            self.current = *s;
        }
    }

    pub fn restart(&mut self) {
        self.current = self.start;
    }

    pub fn accepted(&self) -> bool {
        self.current.accept()
    }

    pub fn current(&self) -> State {
        self.current
    }
}

impl State {

    pub fn new(id: usize, end: bool) -> Self {
        State {
            id: id,
            end: end,
        }
    }

    fn accept(&self) -> bool {
        self.end
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dfa() {

        let s0 = State::new(0, false);
        let s1 = State::new(1, true);

        let mut dfa = Automaton::new(s0);
        dfa.add_state(s1);
        dfa.add_transition(s0, s1, 10);
        dfa.add_transition(s1, s0, 15);

        assert_eq!(s0, dfa.current());

        dfa.consume(1);
        assert_eq!(s0, dfa.current());

        dfa.consume(10);
        assert_eq!(s1, dfa.current());
        assert_eq!(true, dfa.accepted());

        dfa.restart();
        assert_eq!(s0, dfa.current());
    }
}