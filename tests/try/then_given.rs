use beady::scenario;

#[scenario]
#[test]
fn given_then_is_invalid() {
    'given_something: {
        'when_something: {
            'then_something: {
                'given_something: {
                    assert!(true);
                }
            }
        }
    }
}

fn main() {}
