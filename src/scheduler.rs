use std::time;

use bevy_ecs::{prelude::*, schedule::{ScheduleBuildError, ScheduleLabel}};
use r3_core::prelude::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeStep {
    last_time: time::Instant,
}

impl TimeStep {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            last_time: time::Instant::now(),
        }
    }

    pub fn step(&mut self) -> DeltaTime {
        let now = time::Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;
        DeltaTime { delta }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedTimeStep {
    target_time: time::Instant,
    fixed_time: time::Duration,
}

impl FixedTimeStep {
    pub fn new(fixed_time: time::Duration) -> Self {
        Self {
            target_time: time::Instant::now() + fixed_time,
            fixed_time,
        }
    }

    #[inline(always)]
    pub const fn delta(&self) -> DeltaTime {
        DeltaTime {
            delta: self.fixed_time,
        }
    }

    pub fn step(&mut self) -> bool {
        let now = time::Instant::now();

        if now >= self.target_time {
            self.target_time += self.fixed_time;
            true
        } else {
            false
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeltaTime {
    delta: time::Duration,
}

pub struct Scheduler {
    world: World,

    update: TimeStep,
    update_schedules: Vec<Schedule>,
    render_schedules: Vec<Schedule>,
    fixed_update: FixedTimeStep,
    fixed_update_schedules: Vec<Schedule>,
}

impl Scheduler {
    pub fn new(
        render_context: RenderContext,
        mut world: World,
        fixed_update: time::Duration,
    ) -> Self {
        world.insert_resource(render_context);

        Self {
            world,

            update: TimeStep::new(),
            update_schedules: vec![Schedule::new(Update)],
            render_schedules: vec![],
            fixed_update: FixedTimeStep::new(fixed_update),
            fixed_update_schedules: vec![Schedule::new(FixedUpdate)],
        }
    }

    pub fn initialize(&mut self) -> Result<(), ScheduleBuildError> {
        for schedule in self.update_schedules.iter_mut() {
            schedule.initialize(&mut self.world)?;
        }

        for schedule in self.render_schedules.iter_mut() {
            schedule.initialize(&mut self.world)?;
        }

        for schedule in self.fixed_update_schedules.iter_mut() {
            schedule.initialize(&mut self.world)?;
        }

        Ok(())
    }

    pub fn update(&mut self) {
        self.world.insert_resource(self.update.step());

        for schedule in self.update_schedules.iter_mut() {
            schedule.run(&mut self.world);
        }

        self.world.insert_resource(self.fixed_update.delta());

        while self.fixed_update.step() {
            for schedule in self.fixed_update_schedules.iter_mut() {
                schedule.run(&mut self.world);
            }
        }
    }

    pub fn render(&mut self) {
        for schedule in self.render_schedules.iter_mut() {
            schedule.run(&mut self.world);
        }
    }

    #[inline(always)]
    pub const fn world(&self) -> &World {
        &self.world
    }

    #[inline(always)]
    pub const fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    #[inline(always)]
    pub fn get_update_schedule(&mut self, label: impl ScheduleLabel) -> Option<&mut Schedule> {
        self.update_schedules
            .iter_mut()
            .find(|schedule| schedule.label() == label.intern())
    }

    #[inline(always)]
    pub fn get_render_schedule(&mut self, label: impl ScheduleLabel) -> Option<&mut Schedule> {
        self.render_schedules
            .iter_mut()
            .find(|schedule| schedule.label() == label.intern())
    }

    #[inline(always)]
    pub fn get_fixed_update_schedule(
        &mut self,
        label: impl ScheduleLabel,
    ) -> Option<&mut Schedule> {
        self.fixed_update_schedules
            .iter_mut()
            .find(|schedule| schedule.label() == label.intern())
    }

    #[inline(always)]
    pub fn add_pre_update_schedule(&mut self, schedule: Schedule) {
        self.update_schedules.insert(0, schedule);
    }
    #[inline(always)]
    pub fn add_update_schedule(&mut self, schedule: Schedule) {
        self.update_schedules.push(schedule);
    }

    #[inline(always)]
    pub fn add_pre_render_schedule(&mut self, schedule: Schedule) {
        self.render_schedules.insert(0, schedule);
    }
    #[inline(always)]
    pub fn add_render_schedule(&mut self, schedule: Schedule) {
        self.render_schedules.push(schedule);
    }

    #[inline(always)]
    pub fn add_pre_fixed_update_schedule(&mut self, schedule: Schedule) {
        self.fixed_update_schedules.insert(0, schedule);
    }
    #[inline(always)]
    pub fn add_fixed_update_schedule(&mut self, schedule: Schedule) {
        self.fixed_update_schedules.push(schedule);
    }
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Update;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedUpdate;
