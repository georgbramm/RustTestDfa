use std::cmp::Eq;

#[derive(Debug)]
pub struct Automaton<T>
    where T: Eq {
    states: Vec<StateData>,
    edges: Vec<EdgeData<T>>,
    start: StateIndex,
    current: StateIndex,
    end: Vec<StateIndex>,
}

/// An `StateIndex` represents a state in the automaton.
pub type StateIndex = usize;

#[derive(Debug)]
struct StateData {
    first_edge: Option<EdgeIndex>,
}

/// An `EdgeIndex` represents an edge in the automaton.
pub type EdgeIndex = usize;

#[derive(Debug)]
struct EdgeData<T>
    where T: Eq {
    value: T,
    target: StateIndex,
    next_brother_edge: Option<EdgeIndex>, 
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct State {
    id: usize,
    end: bool
}

impl<T> Automaton<T>
    where T: Eq {

    /// Creates a new automaton with 1 state and no edges.
    pub fn new() -> Self {
        let first_state = StateData { first_edge: None };
        Automaton {
            states: vec![first_state],
            edges: Vec::new(),
            start: 0,
            current: 0,
            end: Vec::new(),
        }
    }

    /// Adds a state in the automaton and return it's StateIndex.
    pub fn add_state(&mut self) -> StateIndex {
        let index = self.states.len();
        self.states.push(StateData {
            first_edge: None,
        });
        index
    }

    /// Adds an edge to the automaton, requires the starting state, the ending state
    /// and a value of type T representing the transition value.
    pub fn add_edge(&mut self, source: StateIndex, target: StateIndex, value: T) {
        let edge_index  = self.edges.len();
        let state_data = &mut self.states[source];
        // TODO: Check duplicates
        self.edges.push(EdgeData {
            value,
            target,
            next_brother_edge: state_data.first_edge,
        });
        state_data.first_edge = Some(edge_index);
    }

    /// Sets the starting state of the automaton.
    pub fn set_start(&mut self, new_start: StateIndex) {
        self.start = new_start;
        self.current = new_start;
    }

    /// Adds an accepting state to the automaton.
    pub fn add_end(&mut self, new_end: StateIndex) {
        // TODO: Check duplicates
        self.end.push(new_end);
    }

    /// Consumes a value and advances, if the value is not present in some transition 
    /// then it stays in the same state.
    pub fn consume(&mut self, val: T) {
        let state = &self.states[self.current];
        if let Some(edge) = state.first_edge {
            if self.edges[edge].value == val {
                self.current = self.edges[edge].target;
                return;
            }
            while let Some(edge) = self.edges[edge].next_brother_edge {
                if self.edges[edge].value == val {
                    self.current = self.edges[edge].target;
                    return;
                }   
            }
        }
    }

    /// Restarts the automaton, setting the current state to start.
    pub fn restart(&mut self){
        self.current = self.start;
    }

    /// Returns a boolean value telling if the current state is an accepting state.
    pub fn accepted(&self) -> bool {
        self.end.contains(&self.current)
    }

    /// Returns StateIndex of the current value.
    pub fn current(&self) -> StateIndex {
        self.current
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dfa() {

        let mut dfa = Automaton::new();

        let s0 = dfa.add_state();
        let s1 = dfa.add_state();

        dfa.set_start(s0);
        dfa.add_end(s1);

        assert_eq!(s0, dfa.start);

        dfa.add_edge(s0, s1, 10);
        dfa.add_edge(s1, s0, 15);

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