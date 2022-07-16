use {beady::scenario, std::time::Duration, tokio::time::sleep};

#[scenario]
#[tokio::test]
async fn sleeping_asynchronously() {
    'given_a_sleep: {
        let mut sleep = Box::pin(sleep(Duration::from_millis(1)));

        'when_the_sleep_is_awaited: {
            (&mut sleep).await;

            'then_the_sleep_has_elapsed: {
                assert!(sleep.is_elapsed());
            }
        }
    }
}
