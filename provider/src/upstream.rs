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

/// load (an instance of?) the PostGraphile middleware into node
/// - not sure yet whether we can actually have isolated instances
pub fn init(instance: &str) {
    let instance = instance.to_owned();
    sync_node(move |mut cx| {
        let require: Handle<JsFunction> = cx
            .global()
            .get(&mut cx, "require")?
            .downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let module_path: Handle<JsString> = cx.string("./dist/src/index.js");
        let module = require.call(&mut cx, undefined, vec![module_path])?;
        cx.global()
            .set(&mut cx, format!("query_{}", instance).as_ref(), module)?;
        Ok(())
    })
    .unwrap();
}

pub fn query(instance: &str, query: &'static str) -> QueryResult {
    let instance = instance.to_owned();
    sync_node(move |mut cx| {
        let script = cx.string(format!("query_{}.run", instance));
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

pub(crate) fn remove(instance: &str) {
    let instance = instance.to_owned();
    sync_node(move |mut cx| {
        let undefined = cx.undefined();
        cx.global()
            .set(&mut cx, format!("query_{}", instance).as_ref(), undefined)?;
        Ok(())
    })
    .unwrap();
}
