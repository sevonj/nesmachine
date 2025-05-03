use std::time::Instant;

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
    pub t_last_frame: Instant,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            paused: true,
            command: None,
            t_last_frame: Instant::now(),
        }
    }
}
