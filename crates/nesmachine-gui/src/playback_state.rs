use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
pub enum PlaybackCommand {
    Step,
    Reset,
    Pause,
    Unpause,
}

#[derive(Debug)]
pub struct PlaybackState {
    pub paused: bool,
    pub command: Option<PlaybackCommand>,
    pub t_next_frame: Instant,
    pub breakpoints: HashSet<u16>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            paused: true,
            command: None,
            t_next_frame: Instant::now(),
            breakpoints: HashSet::new(),
        }
    }
}
