// use neon::prelude::{Context, FunctionContext, JsResult, JsString, JsFunction, Handle, JsResultExt, JsUndefined};
// use std::sync::Mutex;
// use std::sync::mpsc;
// use std::{thread, time};

// lazy_static::lazy_static! {
//     static ref APP: Mutex<Option<NativeAdapter>> = Mutex::new(None);
// }

// pub struct App {
//     counter: u64,
//     sender: mpsc::Sender<String>,
// }

// impl App {
//     fn new(sender: mpsc::Sender<String>) -> App {
//         return App{
//             counter: 0,
//             sender,
//         };
//     }

//     fn eventloop(&self, sender: mpsc::Sender<String>) {
//     let mut app = App::new();
//     loop {
//         thread::sleep(time::Duration::from_secs(3));
//         sender.send(format!("hello from eventloop_thread {}", app.counter));
//         app.counter += 1;
//     }
// }
// }

// pub struct NativeAdapter {
//     subscription: JsFunction,
//     receiver: mpsc::Receiver<String>,
// }

// fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let (sender, receiver) = mpsc::channel();
//     let callback = cx.argument::<JsFunction>(0)?;

//     thread::spawn(move || {
//         let app = App::new(sender);
//         app.eventloop();
//     });

//      thread::spawn(move || {
//          let adaptater = NativeAdapter{
//             subscription: callback,
//             receiver,
//         };
//         loop {
//         match adaptater.receiver.recv() {
//             Ok(msg) => {
//                 let args: Vec<Handle<JsString>> = vec![cx.string(""), cx.string(msg.to_string())];
//                 let null = cx.null();
//                 adaptater.subscription.call(&mut cx, null, args)?.downcast::<JsString>().or_throw(&mut cx);
//                 // adaptater.subscription.
//             },
//             Err(_) => panic!("error receiving"),
//         }
//     }
//     });

//     Ok(cx.undefined())
// }

// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node from Rust3"))
// }

// // fn call(mut cx: FunctionContext) -> JsResult<JsString> {
// //     let mut app = APP.lock().unwrap();

// //     app.calls += 1;

// //     // let message = cx.argument::<JsString>(0)?.value();
// //     let callback = cx.argument::<JsFunction>(0)?;

// //     let args: Vec<Handle<JsString>> = vec![cx.string(""), cx.string(app.calls.to_string())];
// //     let null = cx.null();
// //     return callback.call(&mut cx, null, args)?.downcast::<JsString>().or_throw(&mut cx);
// //     // Ok(cx.string("hello node from Rust3"))
// // }

// neon::register_module!(mut cx, {
//     cx.export_function("hello", hello)?;
//     // cx.export_function("call", call)?;
//     Ok(())
// });

mod kernel;

// use std::sync::mpsc::{self, RecvTimeoutError, TryRecvError};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use neon::context::{Context, TaskContext};
// use neon::object::Object;
use neon::result::JsResult;
use neon::task::Task;
use neon::types::{JsFunction, JsUndefined, JsValue};
use neon::{class_definition, declare_types, impl_managed, register_module};
use serde::{Serialize, Deserialize};

/// Placeholder to represent work being done on a Rust thread. It could be
/// reading from a socket or any other long running task.
///
/// Accepts a shutdown channel `shutdown_rx` as an argument. Allows graceful
/// for graceful shutdown by reading from this channel. Shutdown may also be
/// accomplished by waiting for a failed `send` which only occurs when the
/// receiver has hung-up. However, the shutdown channel pattern allows for
/// more control.
///
/// Returns a `Receiver` channel with the data. This is the channel that will
/// be read by the `poll` method.
///
/// It's also useful to note that the `tx` channel created may be cloned if
/// there are multiple threads that produce data to be consumed by Neon.
// fn event_thread(shutdown_rx: mpsc::Receiver<()>) -> mpsc::Receiver<Event> {
//     // Create sending and receiving channels for the event data
//     let (tx, events_rx) = mpsc::channel();

//     // Spawn a thead to continue running after this method has returned.
//     thread::spawn(move || {
//         let mut count = 0.0;

//         // This loop represents the work being performed
//         loop {
//             thread::sleep(Duration::from_secs(1));

//             // Check to see if the thread should be shutdown. `try_recv()`
//             // does not block waiting on data.
//             match shutdown_rx.try_recv() {
//                 // Shutdown if we have received a command or if there is
//                 // nothing to send it.
//                 Ok(_) | Err(TryRecvError::Disconnected) => {
//                     break;
//                 }
//                 // No shutdown command, continue
//                 Err(TryRecvError::Empty) => {}
//             }

//             // Send the event data on the channel. Failure may only occur if
//             // there is no receiver. I.e., if the receiver was destroyed
//             // without shutdown. This should only occur if the shutdown
//             // channel outlives the event channel.
//             tx.send(Event::Tick { count }).expect("Send failed");

//             count += 1.0;
//         }
//     });

//     events_rx
// }

fn app_thread(
    app_receiver: mpsc::Receiver<gui_messages::Message>,
) -> mpsc::Receiver<gui_messages::Message> {
    // Create sending and receiving channels for the event data
    let (gui_sender, gui_receiver) = mpsc::channel();

    // Spawn a thead to continue running after this method has returned.
    thread::spawn(move || {
        let mut app = kernel::App::new(gui_sender, app_receiver);
        app.run();
    });
    gui_receiver
}

/// Reading from a channel `Receiver` is a blocking operation. This struct
/// wraps the data required to perform a read asynchronously from a libuv
/// thread.
pub struct NativeAdaptaterTask(Arc<Mutex<mpsc::Receiver<gui_messages::Message>>>);

/// Implementation of a neon `Task` for `NativeAdaptaterTask`. This task reads
/// from the events channel and calls a JS callback with the data.
impl Task for NativeAdaptaterTask {
    type Output = Option<gui_messages::Message>;
    type Error = String;
    type JsEvent = JsValue;

    /// The work performed on the `libuv` thread. First acquire a lock on
    /// the receiving thread and then return the received data.
    /// In practice, this should never need to wait for a lock since it
    /// should only be executed one at a time by the `NativeAdaptater` class.
    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let rx = self
            .0
            .lock()
            .map_err(|_| "Could not obtain lock on receiver".to_string())?;

        // Attempt to read from the channel. Block for at most 100 ms.
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(message) => Ok(Some(message)),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => Err("Failed to receive message".to_string()),
        }
    }

    /// After the `perform` method has returned, the `complete` method is
    /// scheduled on the main thread. It is responsible for converting the
    /// Rust data structure into a JS object.
    fn complete(
        self,
        mut cx: TaskContext,
        message: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        // Receive the message or return early with the error
        let message = message.or_else(|err| cx.throw_error(&err.to_string()))?;

        // Timeout occured, return early with `undefined
        let message = match message {
            Some(message) => message,
            None => return Ok(JsUndefined::new().upcast()),
        };

        let message = NativeMessage{
            id: "1".to_string(),
            message,
        };

        let js_value = neon_serde::to_value(&mut cx, &message)?;
        Ok(js_value)

        // // Create an empty object `{}`
        // let o = cx.empty_object();

        // // Creates an object of the shape `{ "event": string, ...data }`
        // match event {
        //     Event::Tick { count } => {
        //         let event_name = cx.string("tick");
        //         let event_count = cx.number(count);

        //         o.set(&mut cx, "event", event_name)?;
        //         o.set(&mut cx, "count", event_count)?;
        //     }
        // }

        // let o = cx.empty_object();
        // let event_name = cx.string("event");
        // o.set(&mut cx, "event", event_name)?;

        // let event_data = cx.string(serde_json::to_string(&event).expect("error serializing NativeAdaptater message"));
        // o.set(&mut cx, "data", event_data)?;

        // return Ok(o.upcast());
        // return Ok(cx.string(serde_json::to_string(&event).expect("error serializing NativeAdaptater message")).upcast());
    }
}

/// Rust struct that holds the data required by the `JsNativeAdaptater` class.
pub struct NativeAdaptater {
    // Since the `Receiver` is sent to a thread and mutated, it must be
    // `Send + Sync`. Since, correct usage of the `poll` interface should
    // only have a single concurrent consume, we guard the channel with a
    // `Mutex`.
    events: Arc<Mutex<mpsc::Receiver<gui_messages::Message>>>,

    // Channel used to perform a controlled shutdown of the work thread.
    app_sender: mpsc::Sender<gui_messages::Message>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativeMessage{
    pub id: String,
    pub message: gui_messages::Message,
}

// Implementation of the `JsNativeAdaptater` class. This is the only public
// interface of the Rust code. It exposes the `poll` and `shutdown` methods
// to JS.
declare_types! {
    pub class JsNativeAdaptater for NativeAdaptater {
        // Called by the `JsNativeAdaptater` constructor
        init(_) {
            let (app_sender, app_receiver) = mpsc::channel();

            // Start work in a separate thread
            let rx = app_thread(app_receiver);

            // Construct a new `NativeAdaptater` to be wrapped by the class.
            Ok(NativeAdaptater {
                events: Arc::new(Mutex::new(rx)),
                app_sender,
            })
        }

        // This method should be called by JS to receive data. It accepts a
        // `function (err, data)` style asynchronous callback. It may be called
        // in a loop, but care should be taken to only call it once at a time.
        method poll(mut cx) {
            // The callback to be executed when data is available
            let cb = cx.argument::<JsFunction>(0)?;
            let this = cx.this();

            // Create an asynchronously `NativeAdaptaterTask` to receive data
            let events = cx.borrow(&this, |emitter| Arc::clone(&emitter.events));
            let emitter = NativeAdaptaterTask(events);

            // Schedule the task on the `libuv` thread pool
            emitter.schedule(cb);

            // The `poll` method does not return any data.
            Ok(JsUndefined::new().upcast())
        }

        // The shutdown method may be called to stop the Rust thread. It
        // will error if the thread has already been destroyed.
        method call(mut cx) {
            let this = cx.this();
            let message = cx.argument::<JsValue>(0)?;

            // Unwrap the shutdown channel and send a shutdown command
            let message: NativeMessage = neon_serde::from_value(&mut cx, message)?;
            // let message = kernel::MessageIn{
            //     id: Some("1".to_string()),
            //     data: kernel::MessageData::Tick{ count: 42 },
            // };
            cx.borrow(&this, |emitter| emitter.app_sender.send(message.message))
                .or_else(|err| cx.throw_error(&err.to_string()))?;

            Ok(JsUndefined::new().upcast())
        }
    }
}

// Expose the neon objects as a node module
register_module!(mut cx, {
    // Expose the `JsNativeAdaptater` class as `NativeAdaptater`.
    cx.export_class::<JsNativeAdaptater>("NativeAdaptater")?;

    Ok(())
});
