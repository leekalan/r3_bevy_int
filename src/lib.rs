#![allow(clippy::new_without_default)]
#![allow(unused_imports)]

pub mod scheduler;

use bevy_ecs as ecs;
use r3_core as core;

#[cfg(test)]
mod tests {
    use std::{
        thread::sleep,
        time::{self, Duration},
    };

    use futures::executor::block_on;
    use r3_core::prelude::*;

    use super::{ecs::prelude::*, scheduler::*};

    const FRAME_TIME: time::Duration = time::Duration::from_millis(16);

    #[test]
    fn test() {
        let render_context = block_on(RenderContext::new(RenderContextConfig::default()));

        let mut scheduler = Scheduler::new(render_context, World::new(), FRAME_TIME);

        scheduler
            .get_update_schedule(Update)
            .unwrap()
            .add_systems(run_on_update);

        scheduler
            .get_fixed_update_schedule(FixedUpdate)
            .unwrap()
            .add_systems(run_on_fixed_update);

        scheduler.initialize().unwrap();

        sleep(Duration::from_millis(100));

        scheduler.update();
    }

    fn run_on_update(delta_time: Res<DeltaTime>) {
        println!("Delta Time: {delta_time:?}");
    }

    fn run_on_fixed_update(delta_time: Res<DeltaTime>) {
        println!("Fixed Delta Time: {delta_time:?}");
    }
}
