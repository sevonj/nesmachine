#[derive(Debug)]
pub enum PlaybackCommand {
    Step,
    Reset,
}

#[derive(Debug)]
pub struct PlaybackState {
    pub running: bool,
    pub paused: bool,
    pub command: Option<PlaybackCommand>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            running: true,
            paused: false,
            command: None,
        }
    }
}
