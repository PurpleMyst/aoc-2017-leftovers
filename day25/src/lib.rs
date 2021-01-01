const STATES: usize = 6;
#[derive(Debug, Clone, Copy, Default)]
pub struct StateTransition {
    write: bool,
    move_right: bool,
    next_state: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    zero: StateTransition,
    one: StateTransition,
}

fn load_state_transition(lines: impl Iterator<Item = &'static str>) -> StateTransition {
    let mut last_words = lines.map(|line| line.rsplitn(2, ' ').next().unwrap());

    let write = match last_words.next().unwrap().parse::<u8>().unwrap() {
        0 => false,
        1 => true,
        _ => unreachable!(),
    };

    let move_right = match last_words.next().unwrap() {
        "left" => false,
        "right" => true,
        _ => unreachable!(),
    };

    let next_state = (last_words.next().unwrap().as_bytes()[0] - b'A') as usize;

    StateTransition {
        write,
        move_right,
        next_state,
    }
}

pub fn load_input() -> (usize, [State; STATES]) {
    let mut lines = include_str!("input.txt")
        .lines()
        .map(|line| line.trim_end_matches('.'));

    let steps: usize = lines
        .nth(1)
        .unwrap()
        .split(' ')
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    lines.next().unwrap();

    let mut states = [State::default(); STATES];
    for state in states.iter_mut() {
        lines.next().unwrap();
        lines.next().unwrap();
        let zero = load_state_transition(&mut lines);
        lines.next().unwrap();
        let one = load_state_transition(&mut lines);
        *state = State { zero, one };
        let _ = lines.next();
    }

    (steps, states)
}

#[derive(Debug)]
struct Tape {
    left: Vec<bool>,
    current: Option<bool>,
    right: Vec<bool>,
}

impl Tape {
    fn new() -> Self {
        Self {
            left: Vec::with_capacity(4096),
            current: None,
            right: Vec::with_capacity(4096),
        }
    }

    fn move_left(&mut self) {
        self.right.extend(self.current);
        self.current = Some(self.left.pop().unwrap_or(false));
    }

    fn move_right(&mut self) {
        self.left.extend(self.current);
        self.current = Some(self.right.pop().unwrap_or(false));
    }

    fn current(&self) -> bool {
        self.current.unwrap_or(false)
    }

    fn write(&mut self, value: bool) {
        self.current = Some(value);
    }

    fn checksum(&self) -> usize {
        self.left.iter().filter(|&&value| value).count()
            + (self.current == Some(true)) as usize
            + self.right.iter().filter(|&&value| value).count()
    }
}

#[inline]
pub fn solve() -> usize {
    let (steps, states) = load_input();

    let mut tape = Tape::new();
    let mut state = 0;

    for _ in 0..steps {
        let transition = if tape.current() {
            states[state].one
        } else {
            states[state].zero
        };

        tape.write(transition.write);
        if transition.move_right {
            tape.move_right();
        } else {
            tape.move_left();
        }
        state = transition.next_state;
    }

    tape.checksum()
}
