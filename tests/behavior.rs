use multiagent::{Policy, TerminationCondition};

#[test]
fn closures_can_define_policies() {
    let mut policy = |balance: &u64| (*balance >= 10).then_some("purchase");

    assert_eq!(policy.decide(&12), Some("purchase"));
    assert_eq!(policy.decide(&4), None);
}

#[test]
fn closures_can_define_termination_conditions() {
    let condition = |balance: &u64| *balance == 0;

    assert!(condition.is_terminal(&0));
    assert!(!condition.is_terminal(&1));
}
