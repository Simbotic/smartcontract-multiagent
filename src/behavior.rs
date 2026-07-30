pub trait Policy<Context, Action> {
    fn decide(&mut self, context: &Context) -> Option<Action>;
}

impl<Context, Action, Decide> Policy<Context, Action> for Decide
where
    Decide: FnMut(&Context) -> Option<Action>,
{
    fn decide(&mut self, context: &Context) -> Option<Action> {
        self(context)
    }
}

pub trait TerminationCondition<State> {
    fn is_terminal(&self, state: &State) -> bool;
}

impl<State, Condition> TerminationCondition<State> for Condition
where
    Condition: Fn(&State) -> bool,
{
    fn is_terminal(&self, state: &State) -> bool {
        self(state)
    }
}
