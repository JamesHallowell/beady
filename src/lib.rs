#[macro_export]
macro_rules! beady {
    (SCENARIO($scenario:ident) { $($body:tt)* } $($tail:tt)*) => {
        #[allow(unused_imports, unused_variables)]
        mod $scenario {
            use super::*;
            mod given {
                use super::*;
                $crate::beady!(("Scenario: ", stringify!($scenario)) [] $($body)*);
            }
        }
        $crate::beady!($($tail)*);
    };
    //
    // GIVEN
    //
    (($($path:tt)*) [$($test:tt)*] GIVEN($given:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@given "\n   Given: " ($($path)*) [$($test)*] GIVEN($given) { $($body)* } $($tail)*);
    };
    (@given $output_string:literal ($($path:tt)*) [$($test:tt)*] GIVEN($given:ident) { $($body:tt)* } $($tail:tt)*) => {
        mod $given {
            use super::*;
            mod and_given {
                use super::*;
                $crate::beady!(@and_given ($($path)*, $output_string, stringify!($given)) [$($test)*] $($body)*);
            }
            mod when {
                use super::*;
                $crate::beady!(@given_when ($($path)*, $output_string, stringify!($given)) [$($test)*] $($body)*);
            }
            $crate::beady!(@only_and_or_when $($body)*);
        }
        $crate::beady!(($($path)*) [$($test)*] $($tail)*);
    };
    //
    // WHEN
    //
    (($($path:tt)*) [$($test:tt)*] WHEN($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@when "\n    When: " ($($path)*) [$($test)*] WHEN($when) { $($body)* } $($tail)*);
    };
    (@when $output:literal ($($path:tt)*) [$($test:tt)*] WHEN($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        mod $when {
            use super::*;
            mod and_when {
                use super::*;
                $crate::beady!(@and_when ($($path)*, $output, stringify!($when)) [$($test)*] $($body)*);
            }
            mod then {
                use super::*;
                $crate::beady!(@when_then ($($path)*, $output, stringify!($when)) [$($test)*] $($body)*);
            }
            $crate::beady!(@only_and_or_then $($body)*);
        }
        $crate::beady!(($($path)*) [$($test)*] $($tail)*);
    };
    //
    // AND_GIVEN
    //
    (@and_given ($($path:tt)*) [$($test:tt)*] AND($given:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@given "\n     and: " ($($path)*) [$($test)*] GIVEN($given) { $($body)* } $($tail)*);
        $crate::beady!(@and_given ($($path)*) [$($test)*] $($tail)*);
    };
    (@and_given ($($path:tt)*) [$($test:tt)*] WHEN($given:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@and_given ($($path)*) [$($test)*] $($tail)*);
    };
    (@and_given ($($path:tt)*) [$($test:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(@and_given ($($path)*) [$($test)* $head] $($tail)*);
    };
    (@and_given ($($path:tt)*) [$($test:tt)*]) => {};
    //
    // GIVEN_WHEN
    //
    (@given_when ($($path:tt)*) [$($test:tt)*] WHEN($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@when "\n    When: " ($($path)*) [$($test)*] WHEN($when) { $($body)* });
        $crate::beady!(@given_when ($($path)*) [$($test)*] $($tail)*);
    };
    (@given_when ($($path:tt)*) [$($test:tt)*] AND($given:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@given_when ($($path)*) [$($test)*] $($tail)*);
    };
    (@given_when ($($path:tt)*) [$($test:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(@given_when ($($path)*) [$($test)* $head] $($tail)*);
    };
    (@given_when ($($path:tt)*) [$($test:tt)*]) => {};
    //
    // AND_WHEN
    //
    (@and_when ($($path:tt)*) [$($test:tt)*] AND($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@when "\n     and: " ($($path)*) [$($test)*] WHEN($when) { $($body)* } $($tail)*);
        $crate::beady!(@and_when ($($path)*) [$($test)*] $($tail)*);
    };
    (@and_when ($($path:tt)*) [$($test:tt)*] THEN($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@and_when ($($path)*) [$($test)*] $($tail)*);
    };
    (@and_when ($($path:tt)*) [$($test:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(@and_when ($($path)*) [$($test)* $head] $($tail)*);
    };
    (@and_when ($($path:tt)*) [$($test:tt)*]) => {};
    //
    // WHEN_THEN
    //
    (@when_then ($($path:tt)*) [$($test:tt)*] THEN($then:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@then $then ($($path)*, "\n    Then: ", stringify!($then)) [$($test)*] $($body)*);
        $crate::beady!(@when_then ($($path)*) [$($test)*] $($tail)*);
    };
    (@when_then ($($path:tt)*) [$($test:tt)*] AND($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@when_then ($($path)*) [$($test)*] $($tail)*);
    };
    (@when_then ($($path:tt)*) [$($test:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(@when_then ($($path)*) [$($test)* $head] $($tail)*);
    };
    (@when_then ($($path:tt)*) [$($test:tt)*]) => {};
    //
    // THEN
    //
    (@then $then:ident ($($path:tt)*) [$($test:tt)*] AND($and_then:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@then $and_then ($($path)*, "\n     and: ", stringify!($and_then)) [$($test)*] $($body)*);
        $crate::beady!(@then $then ($($path)*) [$($test)*] $($tail)*);
    };
    (@then $then:ident ($($path:tt)*) [$($test:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(@then $then ($($path)*) [$($test)* $head] $($tail)*);
    };
    (@then $then:ident ($($path:tt)*) [$($test:tt)*]) => {
        #[test]
        fn $then() {
            if let Err(err) = std::panic::catch_unwind(|| { $($test)* }) {
                let panic_message = match err.downcast_ref::<&'static str>() {
                    Some(s) => *s,
                    None => match err.downcast_ref::<String>() {
                        Some(s) => &s[..],
                        None => std::panic::resume_unwind(err),
                    },
                };
                const SCENARIO_GIVEN_WHEN_THEN: &'static str = concat!($($path)*);

                panic!(
                    "-----------------------------------------------------\n\
                    {SCENARIO_GIVEN_WHEN_THEN}\n\
                    \n\
                    {panic_message}\n\
                    -----------------------------------------------------"
                );
            }
        }
    };
    //
    // Checking for bad sections
    //
    (@only_and_or_when AND($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@only_and_or_when $($body)*);
        $crate::beady!(@only_and_or_when $($tail)*);
    };
    (@only_and_or_when WHEN($when:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@only_and_or_when $($tail)*);
    };
    (@only_and_or_when $a:ident($b:ident) { $($c:tt)* } $($d:tt)*) => {
        compile_error!("only AND or WHEN sections are valid inside GIVEN");
    };
    (@only_and_or_when $head:tt $($tail:tt)*) => {
        $crate::beady!(@only_and_or_when $($tail)*);
    };
    (@only_and_or_when) => {};
    (@only_and_or_then AND($then:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@only_and_or_then $($body)*);
        $crate::beady!(@only_and_or_then $($tail)*);
    };
    (@only_and_or_then THEN($then:ident) { $($body:tt)* } $($tail:tt)*) => {
        $crate::beady!(@only_and_or_then $($body)*);
        $crate::beady!(@only_and_or_then $($tail)*);
    };
    (@only_and_or_then $a:ident($b:ident) { $($c:tt)* } $($d:tt)*) => {
        compile_error!("only AND or THEN sections are valid inside WHEN");
    };
    (@only_and_or_then $head:tt $($tail:tt)*) => {
        $crate::beady!(@only_and_or_then $($tail)*);
    };
    (@only_and_or_then) => {};
    //
    // Token munching... 😋
    // 
    (($($path:tt)*) [] $head:tt $($tail:tt)*) => {
        $crate::beady!(($($path)*) [$head] $($tail)*);
    };
    (($($path:tt)*) [$($test_body:tt)*] $head:tt $($tail:tt)*) => {
        $crate::beady!(($($path)*) [$($test_body)* $head] $($tail)*);
    };
    (($($path:tt)*) [$($test_body:tt)*]) => {};
    () => {};
}

#[cfg(test)]
mod tests {
    use super::*;

    beady! {
        SCENARIO(pushing_and_popping_elements_from_a_vector) {
            GIVEN(an_empty_vector) {
                let mut vec = vec![];
                assert!(vec.is_empty());

                WHEN(an_element_is_pushed_to_the_vector) {
                    vec.push(7);

                    THEN(the_vector_has_one_element) {
                        assert_eq!(vec.len(), 1);

                        AND(the_element_is_the_one_pushed) {
                            assert_eq!(vec[0], 7);
                        }
                    }

                    AND(the_element_is_popped) {
                        let popped = vec.pop();

                        THEN(the_vector_is_empty) {
                            assert!(vec.is_empty());

                            AND(the_popped_element_is_the_one_previously_pushed) {
                                assert_eq!(popped, Some(7));
                            }
                        }

                        AND(another_element_is_popped) {
                            let popped = vec.pop();

                            THEN(nothing_was_returned) {
                                assert!(popped.is_none());
                            }
                        }
                    }
                }

                AND(a_list_of_items) {
                    let items = [1, 2, 3];

                    WHEN(the_items_are_pushed_onto_the_vector) {
                        for item in items {
                            vec.push(item);
                        }

                        THEN(the_length_of_the_vector_is_the_number_of_items_pushed) {
                            assert_eq!(vec.len(), items.len());
                        }
                    }
                }
            }
        }

        SCENARIO(clearing_elements_from_a_vector) {
            GIVEN(a_vector_with_multiple_elements) {
                let mut vec = vec![1, 2, 3];

                WHEN(the_vector_is_cleared) {
                    vec.clear();

                    THEN(then_vector_is_empty) {
                        assert!(vec.is_empty());
                    }
                }
            }
        }
    }
}
