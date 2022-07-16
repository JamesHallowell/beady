use beady::scenario;

#[scenario]
fn when_given_is_invalid() {
    'given_something: {
        'when_something: {
            'given_something: {
                assert!(true);
            }
        }
    }
}

fn main() {}
