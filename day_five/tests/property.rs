use proptest::prelude::*;

use std::collections::BTreeSet;

fn expand_ranges(ranges: &[(u64, u64)]) -> BTreeSet<u64> {
    let mut s = BTreeSet::new();
    for &(a, b) in ranges {
        for v in a..=b {
            s.insert(v);
        }
    }
    s
}

proptest! {
    #[test]
    fn merge_ranges_invariants(mut ranges in prop::collection::vec((0u64..100u64, 0u64..100u64), 1..10)) {

        for r in ranges.iter_mut() {
            if r.0 > r.1 { std::mem::swap(&mut r.0, &mut r.1); }
        }

        let merged = day_five::merge_ranges(ranges.clone());

        for i in 1..merged.len() {
            let (s_prev, e_prev) = merged[i-1];
            let (s_cur, e_cur) = merged[i];
            prop_assert!(s_prev <= s_cur);
            prop_assert!(e_prev < s_cur);
            prop_assert!(s_cur <= e_cur);
        }

        let original_set = expand_ranges(&ranges);
        let merged_set = expand_ranges(&merged);
        prop_assert_eq!(original_set, merged_set);
    }
}
