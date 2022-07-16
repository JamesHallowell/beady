use beady::scenario;

#[scenario]
fn given_then_is_invalid() {
    'given_something: {
        'then_something: {
            assert!(true);
        }
    }
}

fn main() {}
