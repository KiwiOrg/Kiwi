use std::{cell::RefCell, future::Future, rc::Rc, task::Poll};

use crate::{
    components, entity,
    global::{OkEmpty, ResultEmpty},
    internal::{component::Entity, executor::EXECUTOR, wit},
};

/// The time, relative to when the application started, in seconds.
/// This can be used to time how long something takes.
pub fn time() -> f32 {
    EXECUTOR.frame_state().time()
}

/// The length of the previous frame, in seconds.
pub fn frametime() -> f32 {
    entity::get_component(entity::resources(), components::core::app::dtime()).unwrap()
}

/// Handle to a "on" listener, which can be canceled by calling `.stop`
pub struct OnHandle(String, u128);
impl OnHandle {
    /// Stops listening
    pub fn stop(self) {
        EXECUTOR.unregister_callback(&self.0, self.1);
    }
}

/// Handle to a "once" listener, which can be canceled by calling `.stop`
pub struct OnceHandle(String, u128);
impl OnceHandle {
    /// Stops listening
    pub fn stop(self) {
        EXECUTOR.unregister_callback_once(&self.0, self.1);
    }
}

/// A trait that abstracts over return types so that you can return an [ResultEmpty] or nothing.
pub trait CallbackReturn {
    #[doc(hidden)]
    fn into_result(self) -> ResultEmpty;
}
impl CallbackReturn for ResultEmpty {
    fn into_result(self) -> ResultEmpty {
        self
    }
}
impl CallbackReturn for () {
    fn into_result(self) -> ResultEmpty {
        OkEmpty
    }
}

/// `on` calls `callback` every time `event` occurs.
///
/// If you only want to be notified once, use [once].
///
/// The `callback` is a `fn`. This can be a closure (e.g. `|args| { ... }`).
pub fn on<R: CallbackReturn>(
    event: &str,
    mut callback: impl FnMut(&Entity) -> R + 'static,
) -> OnHandle {
    wit::event::subscribe(event);
    OnHandle(
        event.to_string(),
        EXECUTOR.register_callback(
            event.to_string(),
            Box::new(move |args| callback(args).into_result()),
        ),
    )
}

/// `once` calls `callback` when `event` occurs, but only once.
///
/// If you want to be notified every time the `event` occurs, use [on].
///
/// The `callback` is a `fn`. This can be a closure (e.g. `|args| { ... }`).
pub fn once<R: CallbackReturn>(
    event: &str,
    callback: impl FnOnce(&Entity) -> R + 'static,
) -> OnceHandle {
    wit::event::subscribe(event);
    OnceHandle(
        event.to_string(),
        EXECUTOR.register_callback_once(
            event.to_string(),
            Box::new(move |args| callback(args).into_result()),
        ),
    )
}

/// Runs the given async block (`future`). This lets your module set up behaviour
/// to run concurrently, like a long-running task. It can return either a [ResultEmpty] or
/// nothing.
///
/// This is similar to [tokio::spawn](https://docs.rs/tokio/latest/tokio/fn.spawn.html),
/// as well as similar functions from other async runtimes.
///
/// # Examples
/// ```
/// run_async(async {
///     notification::broadcast("a title", "hello!");
///     sleep(2.0).await;
///     notification::broadcast("a title", "hello to you too!");
/// });
/// ```
pub fn run_async<R: CallbackReturn>(future: impl Future<Output = R> + 'static) {
    EXECUTOR.spawn(Box::pin(async move { future.await.into_result() }));
}

/// Stops execution of this function until the provided `condition` is true.
/// Useful for waiting for something to happen in the game world.
///
/// This must be used with `.await` in either an `async fn` or an `async` block.
pub async fn block_until(condition: impl Fn() -> bool) {
    std::future::poll_fn(move |_cx| {
        if condition() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await
}

/// Stops execution of this function until `seconds` has passed.
///
/// This must be used with `.await` in either an `async fn` or an `async` block.
pub async fn sleep(seconds: f32) {
    let target_time = time() + seconds;
    block_until(|| time() > target_time).await
}

/// Stops execution of this function until `event` occurs with the specified `condition`.
/// Useful for waiting until a particular event has happened in the game world.
///
/// This must be used with `.await` in either an `async fn` or an `async` block.
pub async fn until_this(event: &str, condition: impl Fn(&Entity) -> bool + 'static) -> Entity {
    let ret = Rc::new(RefCell::new(None));

    fn register_callback(
        event: String,
        condition: impl Fn(&Entity) -> bool + 'static,
        ret: Rc<RefCell<Option<Entity>>>,
    ) {
        once(&event, {
            let event = event.clone();
            move |args: &Entity| {
                if condition(args) {
                    let args = args.clone();
                    *ret.borrow_mut() = Some(args);
                } else {
                    register_callback(event, condition, ret);
                }
                Ok(())
            }
        });
    }
    register_callback(event.to_string(), condition, ret.clone());

    std::future::poll_fn(move |_cx| {
        ret.borrow_mut()
            .take()
            .map(Poll::Ready)
            .unwrap_or(Poll::Pending)
    })
    .await
}

#[deprecated = "Please use `asset::url` instead."]
#[doc(hidden)]
#[cfg(feature = "server")]
pub fn asset_url(path: impl AsRef<str>) -> Option<String> {
    crate::asset::url(path)
}
