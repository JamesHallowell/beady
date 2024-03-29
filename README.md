# beady

A macro for writing tests in a Behaviour Driven (BD) style. Inspired by [Catch2](https://github.com/catchorg/Catch2/blob/devel/docs/test-cases-and-sections.md#bdd-style-test-cases).

- Simple (doesn't really do anything except rearrange your tests)
- Provides helpful output for when tests fail
- Works with `tokio::test` (and other custom test attributes)

## Example

```rust
use beady::scenario;

#[scenario]
#[test]
fn pushing_an_element_to_a_vec() {
    'given_an_empty_vec: {
        let mut vec = vec![];
        
        'when_an_element_is_pushed_to_the_vec: {
            vec.push(7);
            
            'then_the_vec_should_have_one_element: {
                assert_eq!(vec.len(), 1);
                
                'and_then_that_element_should_be_the_pushed_value: {
                    assert_eq!(vec[0], 7);
                }
            }
            
            'and_when_the_vec_is_cleared: {
                vec.clear();
                
                'then_the_vec_should_be_empty: {
                    assert!(vec.is_empty());
                }
            }
        }
    }
}
```

Running `cargo test` we can see that this scenario has generated three tests:

```shell
running 3 tests
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element::and::the_element_should_be_the_pushed_value ... ok
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::and::the_vec_is_cleared::then::the_vec_should_be_empty ... ok
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

And if we make one of our asserts intentionally fail then we see a full description of the failing scenario alongside the panic message:

```shell
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element::and::the_element_should_be_the_pushed_value ... FAILED

failures:

---- pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element::and::the_element_should_be_the_pushed_value stdout ----

Scenario: pushing an element to a vec
   Given: an empty vec
    When: an element is pushed to the vec
    Then: the vec should have one element
     and: the element should be the pushed value

thread 'pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element::and::the_element_should_be_the_pushed_value' panicked at 'assertion failed: `(left == right)`
  left: `7`,
 right: `8`'
```


## Usage

Inspired by the [BDD-style test cases from Catch2](https://github.com/catchorg/Catch2/blob/devel/docs/test-cases-and-sections.md#bdd-style-test-cases), you can annotate a test with `#[scenario]` to make it into a BDD-style test. Within the test you can then use `'given_`, `'when_`, and `'then_` prefixes to label blocks and structure your test cases. Dependent clauses can be specified with the `'and_given_`, `'and_when_`, and `'and_then_` prefixes.