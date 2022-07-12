use std::time::Duration;
use tokio::time::sleep;

#[beady::scenario]
#[tokio::test]
async fn sleeping_asynchronously() {
    #[given(a_sleep)]
    {
        let mut sleep = Box::pin(sleep(Duration::from_millis(1)));

        #[when(the_sleep_is_awaited)]
        {
            (&mut sleep).await;

            #[then(the_sleep_has_elapsed)]
            {
                assert!(sleep.is_elapsed());
            }
        }
    }
}
