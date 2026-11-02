use glam::Vec3;

use crate::core::frame_cached::FrameCache;

/// Frame-rate independent camera position smoothing.
///
/// The original implementation averaged the last N samples, with N derived from
/// `window / frame_time`. At low or uneven frame rates (e.g. 20-30 fps with frame
/// generation) N kept changing from frame to frame, which made the average jump
/// and showed up as camera stutter.
///
/// This version uses time-based exponential smoothing instead: the amount of
/// smoothing per frame depends only on the elapsed time, so uneven frame times
/// no longer cause discontinuities.
pub struct CameraStabilizer {
    window: f32,
    value: Option<Vec3>,
}

impl CameraStabilizer {
    pub const DEFAULT_WINDOW: f32 = 0.3;

    /// Longest frame time taken into account, so a single hitch cannot cause a snap.
    const MAX_FRAME_TIME: f32 = 0.1;
    const MIN_FRAME_TIME: f32 = 1.0 / 480.0;

    pub const fn new(window: f32) -> Self {
        Self {
            window,
            value: None,
        }
    }

    pub fn set_window(&mut self, window: f32) {
        self.window = window;
    }
}

impl FrameCache for CameraStabilizer {
    type Input = Vec3;
    type Output<'a> = Vec3;

    fn update(&mut self, frame_time: f32, input: Self::Input) -> Self::Output<'_> {
        let dt = frame_time.clamp(Self::MIN_FRAME_TIME, Self::MAX_FRAME_TIME);

        // A moving average over `window` seconds lags by about `window / 2`,
        // so use that as the time constant.
        let tau = (self.window * 0.5).max(1e-3);
        let alpha = 1.0 - (-dt / tau).exp();

        let smoothed = match self.value {
            Some(previous) => previous.lerp(input, alpha),
            None => input,
        };

        self.value = Some(smoothed);
        smoothed
    }

    fn get_cached(&mut self, _frame_time: f32, input: Self::Input) -> Self::Output<'_> {
        self.value.unwrap_or(input)
    }

    fn reset(&mut self) {
        self.value = None;
    }
}

impl Default for CameraStabilizer {
    fn default() -> Self {
        Self::new(Self::DEFAULT_WINDOW)
    }
}
