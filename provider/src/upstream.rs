use log::debug;
use nanoid::nanoid;
use nodejs::neon::{
    context::{Context, FunctionContext, TaskContext},
    object::Object,
    prelude::Handle,
    reflect::eval,
    result::{JsResult, NeonResult},
    types::{Finalize, JsFunction, JsNull, JsString, JsUndefined, JsValue, Value},
};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, SyncSender},
        Arc, Mutex,
    },
};

type ResultSender = SyncSender<QueryResult>;

static RESPONSES: Lazy<Arc<Mutex<HashMap<String, ResultSender>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

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
    let id = nanoid!();
    debug!("id: {}, query: {:?}", id, query);
    let (tx, rx) = mpsc::sync_channel::<QueryResult>(0);
    {
        let mut responses = RESPONSES.lock().unwrap();
        responses.insert(id.clone(), tx);
    }
    let id2 = id.clone();
    sync_node(move |mut cx| {
        let script = cx.string("mod.query");
        let func: Handle<JsFunction> = eval(&mut cx, script)?.downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let id: Handle<JsValue> = cx.string(id2).upcast();
        let query: Handle<JsValue> = cx.string(query).upcast();
        let cb: Handle<JsValue> = JsFunction::new(&mut cx, callback)?.upcast();
        func.call(&mut cx, undefined, vec![id, query, cb])?;
        Ok(())
    })
    .unwrap();

    let result = rx.recv().unwrap();
    let mut responses = RESPONSES.lock().unwrap();
    responses.remove(&id);
    result
}

fn callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let id = cx.argument::<JsString>(0)?.value(&mut cx);
    let tx = {
        let responses = RESPONSES.lock().unwrap();
        responses.get(&id).unwrap().clone()
    };
    let error: Handle<JsValue> = cx.argument(1)?;
    if !error.is_a::<JsNull, _>(&mut cx) {
        let error = error.to_string(&mut cx)?.value(&mut cx);
        let result = QueryResult::Err(error);
        debug!("id: {}, result: {:?}", id, result);
        tx.send(result).unwrap();
        return Ok(cx.undefined());
    }
    let res = cx.argument_opt(2);
    let value = if let Some(res) = res {
        res.downcast_or_throw::<JsString, _>(&mut cx)?
            .value(&mut cx)
    } else {
        "".to_string()
    };
    let result = QueryResult::Ok(value);
    debug!("id: {}, result: {:?}", id, result);
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
