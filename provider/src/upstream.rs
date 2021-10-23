use nodejs::neon::{
    context::{Context, FunctionContext, TaskContext},
    object::Object,
    prelude::Handle,
    reflect::eval,
    result::{JsResult, NeonResult},
    types::{Finalize, JsBox, JsError, JsFunction, JsString, JsValue, Value},
};
use std::sync::mpsc;

#[derive(Clone)]
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
        let result = &**func
            .call(&mut cx, undefined, vec![query, cb])?
            .downcast_or_throw::<JsBox<QueryResult>, _>(&mut cx)?;
        Ok(result.clone())
    })
    .unwrap()
}

fn callback(mut cx: FunctionContext) -> JsResult<JsBox<QueryResult>> {
    let err: Handle<JsValue> = cx.argument(0)?;
    if err.is_a::<JsError, _>(&mut cx) {
        let err = err
            .downcast_or_throw::<JsError, _>(&mut cx)?
            .to_string(&mut cx)?
            .value(&mut cx);
        return Ok(cx.boxed(QueryResult::Err(err)));
    }
    let res = cx.argument_opt(1);
    if let Some(res) = res {
        let ok = res
            .downcast_or_throw::<JsString, _>(&mut cx)?
            .value(&mut cx);
        return Ok(cx.boxed(QueryResult::Ok(ok)));
    }
    return Ok(cx.boxed(QueryResult::Err("No value".to_string())));
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
