enum Decision {
    Cooperate,
    Defect
}

type History = Vec<Decision>;


trait Strategy {
    fn decide(other: History, mine: History) -> Decision;
}

trait PartialStrategy {
    fn maybe_decide(other: History, mine: History) -> Option<Decision>;
}

impl<T: Strategy> PartialStrategy for T {
    fn maybe_decide(other: History, mine: History) -> Option<Decision> {
        return Some(Strategy::decide(other, mine));
    }
}

struct TitForTat {}
impl Strategy for TitForTat {
    fn decide(other: History, _: History) -> Decision {
        if other.size() == 0 {
            return Decision::Cooperate;
        }
        else {
            return other.last();
        }
    }
}

struct AlwaysCooperate {}
impl Strategy for AlwaysCooperate {
    fn decide(_: History, _: History) -> Decision {
        return Decision::Cooperate;
    }
}

struct AlwaysDefect {}
impl Strategy for AlwaysDefect {
    fn decide(_: History, _: History) -> Decision {
        return Decision::Defect;
    }
}

fn play<S1: Strategy, S2: Strategy>(p1: S1, p2: S2) {
    let mut history = (Vec::with_capacity(500),
                       Vec::with_capacity(500)
                      );

    for i in 0..500 {
        let d1 = p1.decide(history.2, history.1);
        let d2 = p2.decide(history.1, history.2);
        history.1.push(d1);
        history.2.push(d2);
    }
}

fn main() {
    play(AlwaysCooperate::new(), TitForTat::new());
}
