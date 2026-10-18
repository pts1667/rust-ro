use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

use crate::ImageRef;

/// An image or a text drawn above a unit for `duration_ms`, moved by its motions ("LEVEL UP" rising and fading).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Overhead {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<OverheadText>,
    /// Size of the image; the text is centred below it.
    #[serde(default)]
    pub width: u16,
    #[serde(default)]
    pub height: u16,
    pub duration_ms: u32,
    /// Pixels above the top of the unit.
    #[serde(default)]
    pub offset_y: i16,
    #[serde(default)]
    pub motions: Vec<Motion>,
    #[serde(default)]
    pub audience: Audience,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverheadText {
    pub text: String,
    /// `RRGGBB`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Who sees it: only the player it is about, or everyone around the unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    SelfOnly,
    #[default]
    Area,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    #[serde(flatten)]
    pub kind: MotionKind,
    /// When the motion starts, from the moment the overhead appears. A fade-out without one ends with the overhead,
    /// anything else starts at once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ms: Option<u32>,
    pub ms: u32,
    #[serde(default)]
    pub easing: Easing,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MotionKind {
    Rise { px: f32 },
    FadeIn,
    FadeOut,
    Scale { from: f32, to: f32 },
    /// Grows from nothing past its size, then settles.
    Pop,
    /// Shakes sideways, calming down.
    Shake { px: f32 },
    /// Two hops, the second lower.
    Bounce { px: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Easing {
    Linear,
    EaseIn,
    #[default]
    EaseOut,
    EaseInOut,
}

/// Where and how an overhead is drawn at one instant, relative to its resting place: `dy` is in screen pixels, up is
/// negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub dx: f32,
    pub dy: f32,
    pub alpha: f32,
    pub scale: f32,
}

impl Easing {
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

impl Overhead {
    pub fn image(image: ImageRef, width: u16, height: u16, duration_ms: u32) -> Self {
        Self { image: Some(image), text: None, width, height, duration_ms, offset_y: 0, motions: Vec::new(), audience: Audience::Area }
    }

    pub fn text(text: impl Into<String>, color: Option<&str>, duration_ms: u32) -> Self {
        Self {
            image: None,
            text: Some(OverheadText { text: text.into(), color: color.map(str::to_string) }),
            width: 0,
            height: 0,
            duration_ms,
            offset_y: 0,
            motions: Vec::new(),
            audience: Audience::Area,
        }
    }

    pub fn caption(mut self, text: impl Into<String>, color: Option<&str>) -> Self {
        self.text = Some(OverheadText { text: text.into(), color: color.map(str::to_string) });
        self
    }

    pub fn only_self(mut self) -> Self {
        self.audience = Audience::SelfOnly;
        self
    }

    pub fn offset(mut self, pixels: i16) -> Self {
        self.offset_y = pixels;
        self
    }

    pub fn motion(mut self, kind: MotionKind, ms: u32) -> Self {
        self.motions.push(Motion { kind, start_ms: None, ms, easing: Easing::default() });
        self
    }

    pub fn rise(self, px: f32, ms: u32) -> Self {
        self.motion(MotionKind::Rise { px }, ms)
    }

    pub fn fade_in(self, ms: u32) -> Self {
        self.motion(MotionKind::FadeIn, ms)
    }

    pub fn fade_out(self, ms: u32) -> Self {
        self.motion(MotionKind::FadeOut, ms)
    }

    pub fn pop(self, ms: u32) -> Self {
        self.motion(MotionKind::Pop, ms)
    }

    pub fn scale(self, from: f32, to: f32, ms: u32) -> Self {
        self.motion(MotionKind::Scale { from, to }, ms)
    }

    pub fn shake(self, px: f32, ms: u32) -> Self {
        self.motion(MotionKind::Shake { px }, ms)
    }

    pub fn bounce(self, px: f32, ms: u32) -> Self {
        self.motion(MotionKind::Bounce { px }, ms)
    }

    /// Starts the last motion `start_ms` after the overhead appears.
    pub fn at(mut self, start_ms: u32) -> Self {
        if let Some(motion) = self.motions.last_mut() {
            motion.start_ms = Some(start_ms);
        }
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        if let Some(motion) = self.motions.last_mut() {
            motion.easing = easing;
        }
        self
    }

    pub fn finished(&self, elapsed_ms: u32) -> bool {
        elapsed_ms >= self.duration_ms
    }

    pub fn transform(&self, elapsed_ms: u32) -> Transform {
        let mut transform = Transform { dx: 0.0, dy: 0.0, alpha: 1.0, scale: 1.0 };
        for motion in &self.motions {
            let start = motion.start_ms.unwrap_or(match motion.kind {
                MotionKind::FadeOut => self.duration_ms.saturating_sub(motion.ms),
                _ => 0,
            });
            let linear = if motion.ms == 0 {
                if elapsed_ms >= start {
                    1.0
                } else {
                    0.0
                }
            } else {
                (elapsed_ms as f32 - start as f32) / motion.ms as f32
            }
            .clamp(0.0, 1.0);
            let p = motion.easing.apply(linear);
            match motion.kind {
                MotionKind::Rise { px } => transform.dy -= px * p,
                MotionKind::FadeIn => transform.alpha *= p,
                MotionKind::FadeOut => transform.alpha *= 1.0 - p,
                MotionKind::Scale { from, to } => transform.scale *= from + (to - from) * p,
                MotionKind::Pop => {
                    transform.scale *= if linear < 0.6 { 1.25 * linear / 0.6 } else { 1.25 - 0.25 * (linear - 0.6) / 0.4 };
                }
                MotionKind::Shake { px } => {
                    if linear > 0.0 && linear < 1.0 {
                        transform.dx += px * (elapsed_ms as f32 * 2.0 * PI / 80.0).sin() * (1.0 - linear);
                    }
                }
                MotionKind::Bounce { px } => transform.dy -= px * (linear * 2.0 * PI).sin().abs() * (1.0 - linear),
            }
        }
        transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn a_level_up_rises_pops_and_fades_at_the_end() {
        let overhead = Overhead::image(ImageRef::plugin("levelup.png"), 96, 32, 2000).rise(40.0, 1000).easing(Easing::Linear).pop(300).fade_out(500);
        let start = overhead.transform(0);
        assert!(close(start.dy, 0.0) && close(start.scale, 0.0) && close(start.alpha, 1.0));
        let middle = overhead.transform(500);
        assert!(close(middle.dy, -20.0) && close(middle.scale, 1.0) && close(middle.alpha, 1.0));
        let fading = overhead.transform(1750);
        assert!(close(fading.dy, -40.0) && fading.alpha > 0.0 && fading.alpha < 1.0);
        assert!(close(overhead.transform(2000).alpha, 0.0));
        assert!(overhead.finished(2000));
    }

    #[test]
    fn delayed_motions_wait_for_their_start() {
        let overhead = Overhead::text("Hi", None, 3000).fade_in(200).at(1000);
        assert!(close(overhead.transform(500).alpha, 0.0));
        assert!(close(overhead.transform(1200).alpha, 1.0));
    }

    #[test]
    fn shakes_and_bounces_settle() {
        let overhead = Overhead::text("!", None, 1000).shake(6.0, 400).bounce(10.0, 600);
        let end = overhead.transform(800);
        assert!(close(end.dx, 0.0) && close(end.dy, 0.0));
    }

    #[test]
    fn motions_round_trip_through_json() {
        let overhead = Overhead::text("Hi", Some("ff0000"), 1000).shake(3.0, 200).at(100).only_self();
        let json = serde_json::to_string(&overhead).unwrap();
        assert!(json.contains("\"type\":\"shake\""));
        assert_eq!(serde_json::from_str::<Overhead>(&json).unwrap(), overhead);
    }
}
