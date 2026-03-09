use crate::common::units::Milliseconds;

pub enum PlayerEvent {
    Ready,
    PlaybackStarted,
    PlaybackFinished,
    DeviceLost,
    PositionUpdated(Milliseconds),
    EngineFailed,
}
