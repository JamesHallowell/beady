use beady::scenario;

// The example from Catch2 translated to Rust.
#[scenario]
#[test]
fn vector_can_be_sized_and_resized() {
    'given_an_empty_vector: {
        let mut v = vec![];
        assert!(v.is_empty());

        'when_an_element_is_pushed: {
            v.push("hello");

            'then_the_size_changes: {
                assert_eq!(v.len(), 1);
            }
        }
    }
}

#[scenario]
#[test]
fn large_scenario() {
    'given_an_empty_vector: {
        let mut v = vec![];

        'when_an_element_is_pushed: {
            v.push("hullo");

            'then_the_size_changes: {
                assert_eq!(v.len(), 1);
            }

            'and_when_an_element_is_removed: {
                v.pop();

                'then_the_size_changes: {
                    assert_ne!(v.len(), 1);

                    'and_then_the_vec_is_empty: {
                        assert!(v.is_empty());
                    }
                }
            }
        }

        'and_given_a_vec_with_some_elements: {
            let other = vec![1, 2, 3];

            'when_the_vec_is_merged: {
                v.extend(other);

                'then_the_size_changes: {
                    assert_eq!(v.len(), 3);
                }
            }
        }
    }
}
