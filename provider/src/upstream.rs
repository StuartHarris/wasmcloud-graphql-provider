use log::info;
use nodejs::neon::{
    context::{Context, FunctionContext, TaskContext},
    object::Object,
    prelude::Handle,
    reflect::eval,
    result::{JsResult, NeonResult},
    types::{Finalize, JsFunction, JsNull, JsString, JsUndefined, JsValue, Value},
};
use once_cell::sync::Lazy;
use std::sync::{
    mpsc::{self, Receiver, SyncSender},
    Arc, Mutex,
};

static RESPONSES: Lazy<(
    Arc<Mutex<SyncSender<QueryResult>>>,
    Arc<Mutex<Receiver<QueryResult>>>,
)> = Lazy::new(|| {
    let (tx, rx) = mpsc::sync_channel::<QueryResult>(0);
    (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
});

#[derive(Clone, Debug)]
pub enum QueryResult {
    Ok(String),
    Err(String),
}

impl Finalize for QueryResult {}

/// load the PostGraphile middleware into node
/// - todo: work out if we can have isolated instances
pub fn init(database_url: &str, node_files: &str) {
    let database_url = database_url.to_owned();
    let module_path = format!("{}/dist/src/index.js", node_files);
    sync_node(move |mut cx| {
        // load module
        let require: Handle<JsFunction> = cx
            .global()
            .get(&mut cx, "require")?
            .downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let module_path: Handle<JsString> = cx.string(module_path);
        let module = require.call(&mut cx, undefined, vec![module_path])?;
        cx.global().set(&mut cx, "mod", module)?;

        // call init function
        let script = cx.string("mod.init");
        let func: Handle<JsFunction> = eval(&mut cx, script)?.downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let database_url: Handle<JsValue> = cx.string(database_url).upcast();
        func.call(&mut cx, undefined, vec![database_url])?;
        Ok(())
    })
    .unwrap();
}

pub fn query(query: &str) -> QueryResult {
    let query = query.to_owned();

    sync_node(move |mut cx| {
        let script = cx.string("mod.query");
        let func: Handle<JsFunction> = eval(&mut cx, script)?.downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let query: Handle<JsValue> = cx.string(query).upcast();
        let cb: Handle<JsValue> = JsFunction::new(&mut cx, callback)?.upcast();
        func.call(&mut cx, undefined, vec![query, cb])?;
        Ok(())
    })
    .unwrap();

    let rx = &RESPONSES.1.lock().unwrap();
    let result = rx.recv().unwrap();
    info!("result3: {:?}", result);
    result
}

fn callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let error: Handle<JsValue> = cx.argument(0)?;
    if !error.is_a::<JsNull, _>(&mut cx) {
        let error = error.to_string(&mut cx)?.value(&mut cx);
        let result = QueryResult::Err(error);
        info!("result1: {:?}", result);
        let tx = (*RESPONSES.0.lock().unwrap()).clone();
        tx.send(result).unwrap();
        return Ok(cx.undefined());
    }
    let res = cx.argument_opt(1);
    let value = if let Some(res) = res {
        res.downcast_or_throw::<JsString, _>(&mut cx)?
            .value(&mut cx)
    } else {
        "".to_string()
    };
    info!("in callback");
    let result = QueryResult::Ok(value);
    info!("result2: {:?}", result);
    let tx = (*RESPONSES.0.lock().unwrap()).clone();
    tx.send(result).unwrap();
    Ok(cx.undefined())
}

pub fn remove() {
    sync_node(move |mut cx| {
        let undefined = cx.undefined();
        cx.global().set(&mut cx, "mod", undefined)?;
        Ok(())
    })
    .unwrap();
}

fn sync_node<T: Send + 'static>(
    f: impl FnOnce(TaskContext) -> NeonResult<T> + Send + 'static,
) -> Option<T> {
    let nodejs_channel = nodejs::channel();
    let (tx, rx) = mpsc::sync_channel::<T>(0);
    nodejs_channel.send(move |cx| {
        let val = f(cx)?;
        tx.send(val).unwrap();
        Ok(())
    });
    rx.recv().ok()
}
