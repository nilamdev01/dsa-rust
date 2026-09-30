impl Solution {
    pub fn poor_pigs(
        buckets: i32,
        minutes_to_die: i32,
        minutes_to_test: i32,
    ) -> i32 {
        let rounds = minutes_to_test / minutes_to_die;
        let states = rounds + 1;

        let mut pigs = 0;
        let mut possibilities = 1;

        while possibilities < buckets {
            possibilities *= states;
            pigs += 1;
        }

        pigs
    }
}