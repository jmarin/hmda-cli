pub struct LarAction {
    pub preapproval: i8,
    pub action_taken_type: i8,
    pub action_taken_date: i32,
}

impl LarAction {
    pub fn sample_lar_action() -> LarAction {
        LarAction {
            preapproval: 1,
            action_taken_type: 1,
            action_taken_date: 20180721,
        }
    }
}
