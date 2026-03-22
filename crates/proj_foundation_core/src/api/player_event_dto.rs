use crate::common::units::Milliseconds;

pub enum PlayerEventDto {
    Ready,
    PlaybackStarted,
    PlaybackFinished,
    DeviceLost,
    PositionUpdated(Milliseconds),
    EngineFailed,
}
