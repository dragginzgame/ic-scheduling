use super::*;

#[test]
fn saturation_preserves_owners_and_only_returned_slots_are_reused() {
    let mut slots = Slots::<_, 128>::new();
    for value in 0..128 {
        assert_eq!(slots.insert(value), Ok(value));
    }
    assert_eq!(slots.insert(999), Err(999));
    assert_eq!(slots.take(usize::MAX), None);
    assert_eq!(slots.take(17), Some(17));
    assert_eq!(slots.take(17), None);
    assert_eq!(slots.insert(999), Ok(17));
    for index in 0..128 {
        assert_eq!(
            slots.take(index),
            Some(if index == 17 { 999 } else { index })
        );
    }
    assert_eq!(slots.insert(1000), Ok(0));
    assert_eq!(slots.0.len(), 128);
}

#[test]
fn out_of_order_terminals_preserve_capture_ownership_across_slot_reuse() {
    use std::rc::Rc;

    // Every completion order for three concurrent flights. A slot may be
    // reused only after its terminal owner returns; the other flights stay
    // live throughout replacement admission and completion.
    for first in 0..3 {
        for second in 0..3 {
            if second == first {
                continue;
            }
            let third = 3 - first - second;
            let captures = [Rc::new(()), Rc::new(()), Rc::new(())];
            let mut slots = Slots::<_, 3>::new();
            for capture in &captures {
                slots.insert(Rc::clone(capture)).unwrap();
            }
            let mut replacements = Vec::new();
            for index in [first, second, third] {
                let capture = slots.take(index).unwrap();
                assert!(Rc::ptr_eq(&capture, &captures[index]));
                assert!(slots.take(index).is_none());
                drop(capture);
                assert_eq!(Rc::strong_count(&captures[index]), 1);

                let replacement = Rc::new(());
                assert_eq!(slots.insert(Rc::clone(&replacement)), Ok(index));
                assert_eq!(Rc::strong_count(&replacement), 2);
                replacements.push(replacement);
            }
            drop(slots);
            assert!(
                captures
                    .iter()
                    .all(|capture| Rc::strong_count(capture) == 1)
            );
            assert!(
                replacements
                    .iter()
                    .all(|capture| Rc::strong_count(capture) == 1)
            );
        }
    }
}
