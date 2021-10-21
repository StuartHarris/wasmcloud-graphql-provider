use nodejs::neon::{
    context::{Context, FunctionContext, TaskContext},
    object::Object,
    prelude::Handle,
    reflect::eval,
    result::{JsResult, NeonResult},
    types::{JsError, JsFunction, JsString, JsUndefined, JsValue, Value},
};
use std::{sync::mpsc, thread, time::Duration};

pub fn start() {
    require();
    thread::sleep(Duration::from_millis(1000));
    for _ in 1..=4 {
        query(
            r#"
			query MyQuery {
				productLists {
					nodes {
						nodeId
						userId
						id
						title
					}
				}
			}"#,
        );
    }
    thread::park();
}

fn require() {
    sync_node(move |mut cx| {
        let require: Handle<JsFunction> = cx
            .global()
            .get(&mut cx, "require")?
            .downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let module_path: Handle<JsString> = cx.string("./dist/src/index.js");
        let module = require.call(&mut cx, undefined, vec![module_path])?;
        cx.global().set(&mut cx, "query", module)?;
        Ok(())
    })
    .unwrap();
}

fn query(query: &'static str) {
    sync_node(move |mut cx| {
        let script = cx.string("query.run");
        let func: Handle<JsFunction> = eval(&mut cx, script)?.downcast_or_throw(&mut cx)?;
        let undefined = cx.undefined();
        let query: Handle<JsValue> = cx.string(query).upcast();
        let cb: Handle<JsValue> = JsFunction::new(&mut cx, callback)?.upcast();
        func.call(&mut cx, undefined, vec![query, cb])?;
        Ok(())
    })
    .unwrap();
}

fn callback(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let err: Handle<JsValue> = cx.argument(0)?;
    if err.is_a::<JsError, _>(&mut cx) {
        println!(
            "error: {}",
            err.downcast_or_throw::<JsError, _>(&mut cx)?
                .to_string(&mut cx)?
                .value(&mut cx)
        );
    }
    let res = cx.argument_opt(1);
    if let Some(res) = res {
        println!(
            "result: {}",
            res.downcast_or_throw::<JsString, _>(&mut cx)?
                .value(&mut cx)
        );
    }
    Ok(cx.undefined())
}

pub fn sync_node<T: Send + 'static>(
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
