# beady

A macro for writing tests in a Behaviour Driven (BD) style. Inspired by [Catch2](https://github.com/catchorg/Catch2/blob/devel/docs/test-cases-and-sections.md#bdd-style-test-cases).

- Simple (doesn't really do anything except reorganise your tests)
- Readable output to help diagnose failures
- Works with `tokio::test` (and other test attributes)

## Example

```rust
use beady::scenario;

#[scenario]
fn pushing_an_element_to_a_vec() {
    #[given(an_empty_vec)] {
        let mut vec = vec![];
        
        #[when(an_element_is_pushed_to_the_vec)] {
            vec.push(7);
            
            #[then(the_vec_should_have_one_element)] {
                assert_eq!(vec.len(), 1);
                
                #[and_then(that_element_should_be_the_pushed_value)] {
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
```

Running `cargo test -- --nocapture` will output the following:

```shell
     Running tests/basic.rs (target/debug/deps/basic-818c7ee31bf8afc8)

running 3 tests
--------------------------------------------------------------------------------
Scenario: pushing an element to a vec
   Given: an empty vec
    When: an element is pushed to the vec
    Then: the vec should have one element
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
Scenario: pushing an element to a vec
   Given: an empty vec
    When: an element is pushed to the vec
     and: the vec is cleared
    Then: the vec should be empty
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
Scenario: pushing an element to a vec
   Given: an empty vec
    When: an element is pushed to the vec
    Then: the vec should have one element
     and: that element should be the pushed value
--------------------------------------------------------------------------------
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::the_vec_should_have_one_element ... ok
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::and::the_vec_is_cleared::then::the_vec_should_be_empty ... ok
test pushing_an_element_to_a_vec::given::an_empty_vec::when::an_element_is_pushed_to_the_vec::then::that_element_should_be_the_pushed_value ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

## Usage

Inspired by the [BDD-style test cases from Catch2](https://github.com/catchorg/Catch2/blob/devel/docs/test-cases-and-sections.md#bdd-style-test-cases), you can annotate a function with `#[scenario]` to make it a test, and then use the `#[given]`, `#[when]`, and `#[then]` attributes to describe the test. Dependent clauses can be specified with the `#[and_given]`, `#[and_when]`, and `#[and_then]` attributes.

### `#[scenario]`

By default the `#[scenario]` attribute will generate tests like:

```rust
#[test]
fn foo() { ... }
```

If you want to use another test attribute you can specifiy it after the `#[scenario]` attribute like so:

```rust
#[scenario]
#[tokio::test]
async fn my_scenario() { ... }
```

Which will generate tests like:

```rust
#[tokio::test]
async fn foo() { ... }
```