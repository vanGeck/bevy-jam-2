use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Default, Clone)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub index: usize,
    pub nr_frames: usize,
    pub ping_pong: bool,
}

impl AnimationTimer {
    pub fn for_player() -> Self {
        AnimationTimer {
            timer: Timer::new(Duration::from_millis(50), true),
            index: 2,
            nr_frames: 5,
            ping_pong: true,
        }
    }

    pub fn tick(&mut self, delta: Duration) -> usize {
        self.timer.tick(delta);
        self.index = (self.index + self.timer.times_finished_this_tick() as usize).rem_euclid(
            if self.ping_pong {
                self.nr_frames * 2
            } else {
                self.nr_frames
            },
        );
        if self.index < self.nr_frames {
            self.index
        } else {
            self.nr_frames * 2 - self.index - 1
        }
    }
}
