use gloo_timers::callback::Interval;
use yew::prelude::*;

/// A hook that increments a counter every `interval_ms` milliseconds,
/// causing the component to re-render so relative timestamps stay fresh.
/// No HTTP calls are made — this is purely a local tick.
#[hook]
pub fn use_clock_tick(interval_ms: u32) -> u32 {
    let tick = use_state(|| 0u32);

    {
        let tick = tick.clone();
        use_effect_with(interval_ms, move |&ms| {
            let interval = Interval::new(ms, move || {
                tick.set(*tick + 1);
            });

            // Keep the interval alive for the lifetime of the component.
            // Dropping it would cancel it, so we move it into the cleanup
            // closure and drop it there instead.
            move || drop(interval)
        });
    }

    *tick
}
