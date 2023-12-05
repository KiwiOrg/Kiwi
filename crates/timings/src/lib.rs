use std::{collections::VecDeque, sync::atomic::AtomicBool, time::Duration};

use ambient_ecs::{components, Debuggable, FrameEvent, Resource, System, World, WorldContext};
use ambient_sys::time::Instant;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use winit::event::{DeviceEvent, Event, WindowEvent};

const MAX_SAMPLES: usize = 128;
static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

/// Frame events being timed (in order!)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum TimingEventType {
    Input,
    ScriptingStarted,
    ScriptingFinished,
    ClientSystemsStarted,
    ClientSystemsFinished,
    DrawingWorld,
    DrawingUI,
    SubmittingGPUCommands,
    RenderingFinished,
}
impl TimingEventType {
    const COUNT: usize = Self::last().idx() + 1;

    const fn last() -> Self {
        Self::RenderingFinished
    }

    const fn idx(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimingEvent {
    event_type: TimingEventType,
    time: Instant,
}
impl From<TimingEventType> for TimingEvent {
    fn from(event_type: TimingEventType) -> Self {
        Self {
            event_type,
            time: Instant::now(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FrameTimings {
    event_times: [Option<Instant>; TimingEventType::COUNT],
}
impl FrameTimings {
    pub fn input_to_rendered(&self) -> Option<Duration> {
        self.event_times[TimingEventType::Input as usize]
            .zip(self.event_times[TimingEventType::RenderingFinished as usize])
            .map(|(input, rendered)| rendered - input)
    }
}
impl std::fmt::Display for FrameTimings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last = None;
        for (time, idx) in self
            .event_times
            .iter()
            .enumerate()
            .filter_map(|(idx, t)| t.zip(Some(idx)))
        {
            let event_type = TimingEventType::from_usize(idx).unwrap();
            if let Some(last) = last {
                write!(f, " <- {:?} -> ", time - last)?;
            }
            write!(f, "{:?}", event_type)?;
            last = Some(time);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Frame {
    last_event_type: TimingEventType,
    event_times: [Option<Instant>; TimingEventType::COUNT],
}
impl Frame {
    fn timings(&self) -> FrameTimings {
        FrameTimings {
            event_times: self.event_times,
        }
    }

    fn should_accept_event(&self, event_type: TimingEventType) -> bool {
        // if it simply is the next event
        self.last_event_type.idx() + 1 == event_type.idx() ||
        // or if it is repeated input event (there can be multiple input events)
         (self.last_event_type == TimingEventType::Input && event_type == TimingEventType::Input) ||
        // or we don't have app systems finished event when rendering is finished (we don't have the callback in the browser)
         (self.last_event_type == TimingEventType::SubmittingGPUCommands && event_type == TimingEventType::RenderingFinished)
    }

    fn is_accepting_input(&self) -> bool {
        self.last_event_type == TimingEventType::Input
    }

    fn is_finished(&self) -> bool {
        self.last_event_type == TimingEventType::last()
    }

    fn process_event(&mut self, event: TimingEvent) {
        if self.should_accept_event(event.event_type) {
            self.event_times[event.event_type.idx()] =
                self.event_times[event.event_type.idx()].or(Some(event.time));
            self.last_event_type = event.event_type;
        }
    }
}
impl Default for Frame {
    fn default() -> Self {
        Self {
            last_event_type: TimingEventType::Input,
            event_times: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ProcessTimingEventsSystem {
    frames: VecDeque<Frame>,
}
impl Default for ProcessTimingEventsSystem {
    fn default() -> Self {
        // we normally have at most 2 frames in flight
        let mut frames = VecDeque::with_capacity(2);
        frames.push_back(Default::default());
        Self { frames }
    }
}
impl System for ProcessTimingEventsSystem {
    fn run(&mut self, world: &mut World, _event: &FrameEvent) {
        // only process timing events in the app world so that they are available for the debugger
        if world.context() != WorldContext::App {
            return;
        }

        let mut pending_samples = Vec::new();
        let timings = &world.resource(reporter()).receiver;
        for event in timings.try_iter() {
            for f in self.frames.iter_mut() {
                f.process_event(event);
            }

            if !self.frames.front().unwrap().is_accepting_input() {
                self.frames.push_front(Default::default());
            }

            while let Some(f) = self.frames.back() {
                if f.is_finished() {
                    let timings = self.frames.pop_back().unwrap().timings();
                    pending_samples.push(timings);
                } else {
                    break;
                }
            }
        }

        if !pending_samples.is_empty() {
            let samples = world.resource_mut(samples());
            for sample in pending_samples {
                while samples.len() + 1 >= MAX_SAMPLES {
                    samples.pop_front();
                }
                samples.push_back(sample);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Reporter {
    sender: flume::Sender<TimingEvent>,
    receiver: flume::Receiver<TimingEvent>,
}
impl Default for Reporter {
    fn default() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }
}
impl Reporter {
    pub fn report_event(&self, event: impl Into<TimingEvent>) {
        if ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.sender.send(event.into()).ok();
        }
    }

    pub fn reporter(&self) -> ThinReporter {
        if ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            ThinReporter::Enabled(self.sender.clone())
        } else {
            ThinReporter::Disabled
        }
    }
}

pub enum ThinReporter {
    Disabled,
    Enabled(flume::Sender<TimingEvent>),
}
impl ThinReporter {
    pub fn report_event(&self, event: impl Into<TimingEvent>) {
        match self {
            Self::Disabled => {}
            Self::Enabled(sender) => {
                sender.send(event.into()).ok();
            }
        }
    }
}

components!("timings", {
    @[Debuggable, Resource]
    reporter: Reporter,

    @[Debuggable, Resource]
    samples: VecDeque<FrameTimings>,
});

#[derive(Debug)]
struct ClientWorldTimingSystem<const EVENT_TYPE: usize>;
impl<const EVENT_TYPE: usize> System for ClientWorldTimingSystem<EVENT_TYPE> {
    fn run(&mut self, world: &mut World, _event: &FrameEvent) {
        if world.context() != WorldContext::Client {
            return;
        }

        // emit timing events in the client world
        let time = Instant::now();
        let event_type = TimingEventType::from_usize(EVENT_TYPE).unwrap();
        world
            .resource(reporter())
            .report_event(TimingEvent { time, event_type });
    }
}

pub const fn on_started_timing_system() -> impl System {
    const EVENT_TYPE: usize = TimingEventType::ClientSystemsStarted as usize;
    ClientWorldTimingSystem::<EVENT_TYPE> {}
}

pub const fn on_finished_timing_system() -> impl System {
    const EVENT_TYPE: usize = TimingEventType::ClientSystemsFinished as usize;
    ClientWorldTimingSystem::<EVENT_TYPE> {}
}

#[derive(Debug)]
struct SystemWrapper<S> {
    system: S,
    on_started: TimingEventType,
    on_finished: TimingEventType,
}
impl<S, E> System<E> for SystemWrapper<S>
where
    S: System<E>,
{
    fn run(&mut self, world: &mut World, event: &E) {
        let r = world.resource(reporter()).reporter();
        r.report_event(TimingEvent::from(self.on_started));
        self.system.run(world, event);
        r.report_event(TimingEvent::from(self.on_finished));
    }
}

pub fn wrap_system<E>(
    system: impl System<E>,
    on_started: TimingEventType,
    on_finished: TimingEventType,
) -> impl System<E> {
    SystemWrapper {
        system,
        on_started,
        on_finished,
    }
}

#[derive(Debug)]
pub struct InputTimingSystem;
impl System<Event<'static, ()>> for InputTimingSystem {
    fn run(&mut self, world: &mut World, event: &Event<'static, ()>) {
        if is_user_input_event(event) {
            world
                .resource_mut(reporter())
                .report_event(TimingEventType::Input);
        }
    }
}

fn is_user_input_event(event: &Event<'static, ()>) -> bool {
    matches!(
        event,
        Event::WindowEvent {
            event: WindowEvent::ModifiersChanged(_)
                | WindowEvent::KeyboardInput { .. }
                | WindowEvent::MouseInput { .. }
                | WindowEvent::MouseWheel { .. },
            ..
        } | Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { .. },
            ..
        }
    )
}