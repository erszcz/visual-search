extern crate clock_ticks;

pub struct FrameCounter {
    _fps: u32,
    frame_len_ns: u64,
    last_ns: u64
}

impl FrameCounter {

    pub fn from_fps(fps: u32) -> FrameCounter {
        FrameCounter::from_fps_and_last(fps, clock_ticks::precise_time_ns())
    }

    pub fn update(&mut self) -> FrameUpdate {
        FrameCounter::update_with_now(self, clock_ticks::precise_time_ns())
    }

    #[inline]
    fn from_fps_and_last(fps: u32, last_ns: u64) -> FrameCounter {
        FrameCounter { _fps: fps,
                       frame_len_ns: (1e9 / fps as f64) as u64,
                       last_ns: last_ns }
    }

    #[inline]
    fn update_with_now(&mut self, now_ns: u64) -> FrameUpdate {
        let elapsed_ns = now_ns - self.last_ns;
        if elapsed_ns > self.frame_len_ns {
            self.last_ns = now_ns - now_ns % self.frame_len_ns;
            FrameUpdate::NewFrame {
                elapsed_ns: elapsed_ns,
                elapsed_frames: elapsed_ns / self.frame_len_ns
            }
        } else {
            FrameUpdate::OldFrame
        }
    }

}

#[derive(PartialEq, Eq, Debug)]
pub enum FrameUpdate {
    NewFrame { elapsed_frames: u64,
               elapsed_ns: u64 },
    OldFrame
}

#[test]
fn with_25fps_frame_is_40ms_long() {
    let fc = FrameCounter::from_fps_and_last(25, 0);
    assert_eq!(fc.frame_len_ns / 1000 / 1000, 40);
}

#[test]
fn with_25fps_after_41ms_its_a_new_frame() {
    let elapsed_ns = 41 * 1000 * 1000;
    let expected =
        FrameUpdate::NewFrame { elapsed_ns: elapsed_ns,
                                elapsed_frames: 1 };
    with_fps_after_some_ms(25, elapsed_ns, expected);
}

#[test]
fn with_25fps_after_81ms_its_a_new_frame() {
    let elapsed_ns = 81 * 1000 * 1000;
    let expected =
        FrameUpdate::NewFrame { elapsed_ns: elapsed_ns,
                                elapsed_frames: 2 };
    with_fps_after_some_ms(25, elapsed_ns, expected);
}

#[test]
fn with_25fps_after_39ms_its_the_same_frame() {
    let elapsed_ns = 39 * 1000 * 1000;
    let expected = FrameUpdate::OldFrame;
    with_fps_after_some_ms(25, elapsed_ns, expected);
}

#[cfg(test)]
fn with_fps_after_some_ms(fps: u32, elapsed_ns: u64, expected: FrameUpdate) {
    let mut fc = FrameCounter::from_fps_and_last(fps, 0);
    let new_frame = fc.update_with_now(elapsed_ns);
    assert_eq!(expected, new_frame);
}

#[test]
fn with_old_frame_update_status_last_time_is_not_updated() {
    let fake_now_ns = 0;
    let mut fc = FrameCounter::from_fps_and_last(25, fake_now_ns);
    let new_frame = fc.update_with_now(39 * 1000 * 1000);
    assert_eq!(FrameUpdate::OldFrame, new_frame);
    assert_eq!(fake_now_ns, fc.last_ns);
}
