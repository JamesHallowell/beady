// The example from Catch2 translated to Rust.
#[beady::scenario]
fn vector_can_be_sized_and_resized() {
    #[given(an_empty_vector)]
    {
        let mut v = vec![];

        #[when(push_is_called)]
        {
            v.push("hullo");

            #[then(the_size_changes)]
            {
                assert_eq!(v.len(), 1);
            }
        }
    }
}

#[beady::scenario]
fn nested() {
    #[given(a)]
    {
        let a = 5;

        #[and_given(b)]
        {
            let b = 3;

            #[when(a_is_added_to_b)]
            {
                let c = a + b;

                #[then(the_result_is_a_plus_b)]
                {
                    assert_eq!(c, a + b);
                }

                #[and_when(c_is_multiplied_by_b)]
                {
                    let d = c * b;

                    #[then(the_result_is_b_times_c)]
                    {
                        assert_eq!(d, b * c);
                    }
                }
            }
        }
    }
}

#[beady::scenario]
fn pushing_an_element_to_a_vec() {
    #[given(an_empty_vec)] {
        let mut vec = vec![];

        #[when(an_element_is_pushed_to_the_vec)] {
            vec.push(7);

            #[then(the_vec_should_have_one_element)] {
                assert_eq!(vec.len(), 1);

                #[and_then(the_element_should_be_the_pushed_value)] {
                    assert_eq!(vec[0], 7);
                }
            }

            #[and_when(the_vec_is_cleared)] {
                vec.clear();

                #[then(the_vec_should_be_empty)] {
                    assert!(vec.is_empty());
                }
            }
        }
    }
}