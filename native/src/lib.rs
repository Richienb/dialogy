extern crate tinyfiledialogs as dialogs;
use neon::prelude::*;

use dialogs::{DefaultColorValue, MessageBoxIcon, OkCancel, YesNo};

fn color(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_color = &*context.argument::<JsString>(1)?.value(&mut context);

	let color: String;
	match dialogs::color_chooser_dialog(title, DefaultColorValue::Hex(default_color)) {
		Some((hex, _)) => color = hex,
		None => color = default_color.to_string(),
	}

	Ok(context.string(color))
}

fn alert(mut context: FunctionContext) -> JsResult<JsBoolean> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let message = &*context.argument::<JsString>(1)?.value(&mut context);
	let icon = match &*context.argument::<JsString>(2)?.value(&mut context) {
		"info" => MessageBoxIcon::Info,
		"warning" => MessageBoxIcon::Warning,
		"error" => MessageBoxIcon::Error,
		"question" => MessageBoxIcon::Question,
		_ => unreachable!(),
	};
	let buttons = &*context.argument::<JsString>(3)?.value(&mut context);
	let default_value = match context.argument_opt(4) {
		Some(argument) => argument
			.downcast::<JsBoolean, _>(&mut context)
			.or_throw(&mut context)?
			.value(&mut context),
		None => unreachable!(),
	};

	Ok(context.boolean(match buttons {
		"yesno" => {
			dialogs::message_box_yes_no(
				title,
				message,
				icon,
				match default_value {
					true => YesNo::Yes,
					false => YesNo::No,
				},
			) == YesNo::Yes
		}
		"okcancel" => {
			dialogs::message_box_ok_cancel(
				title,
				message,
				icon,
				match default_value {
					true => OkCancel::Ok,
					false => OkCancel::Cancel,
				},
			) == OkCancel::Ok
		}
		"ok" => {
			dialogs::message_box_ok(title, message, icon);
			true
		}
		_ => unreachable!(),
	}))
}

fn save_file(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);

	Ok(context.string(dialogs::save_file_dialog(title, default_value).unwrap_or("".to_string())))
}

fn save_file_filter(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);
	let filter_patterns = context
		.argument::<JsArray>(2)?
		.to_vec(&mut context)?
		.into_iter()
		.map(|value| {
			value
				.downcast::<JsString, _>(&mut context)
				.or_throw(&mut context)
				.map(|value| value.value(&mut context))
		})
		.collect::<Result<Vec<_>, _>>()?;
	let filter_patterns = filter_patterns
		.iter()
		.map(|value| value.as_str())
		.collect::<Vec<_>>();
	let filter_description = &*context.argument::<JsString>(3)?.value(&mut context);

	Ok(context.string(
		dialogs::save_file_dialog_with_filter(
			title,
			default_value,
			&filter_patterns,
			filter_description,
		)
		.unwrap_or("".to_string()),
	))
}

fn open_file(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);

	Ok(context
		.string(dialogs::open_file_dialog(title, default_value, None).unwrap_or("".to_string())))
}

fn open_file_filter(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);
	let filter_patterns = context
		.argument::<JsArray>(2)?
		.to_vec(&mut context)?
		.into_iter()
		.map(|value| {
			value
				.downcast::<JsString, _>(&mut context)
				.or_throw(&mut context)
				.map(|value| value.value(&mut context))
		})
		.collect::<Result<Vec<_>, _>>()?;
	let filter_patterns = filter_patterns
		.iter()
		.map(|value| value.as_str())
		.collect::<Vec<_>>();
	let filter_description = &*context.argument::<JsString>(3)?.value(&mut context);

	Ok(context.string(
		dialogs::open_file_dialog(
			title,
			default_value,
			Some((&filter_patterns, filter_description)),
		)
		.unwrap_or("".to_string()),
	))
}

fn vec_to_array(mut context: FunctionContext, vec: Vec<String>) -> JsResult<JsArray> {
	let result = JsArray::new(&mut context, vec.len() as u32);
	for (index, value) in vec.iter().enumerate() {
		let string = context.string(value);
		result.set(&mut context, index as u32, string).unwrap();
	}
	Ok(result)
}

fn open_file_multiple(mut context: FunctionContext) -> JsResult<JsArray> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);

	vec_to_array(
		context,
		dialogs::open_file_dialog_multi(title, default_value, None).unwrap_or(vec![]),
	)
}

fn open_file_multiple_filter(mut context: FunctionContext) -> JsResult<JsArray> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);
	let filter_patterns = context
		.argument::<JsArray>(2)?
		.to_vec(&mut context)?
		.into_iter()
		.map(|value| {
			value
				.downcast::<JsString, _>(&mut context)
				.or_throw(&mut context)
				.map(|value| value.value(&mut context))
		})
		.collect::<Result<Vec<_>, _>>()?;
	let filter_patterns = filter_patterns
		.iter()
		.map(|value| value.as_str())
		.collect::<Vec<_>>();
	let filter_description = &*context.argument::<JsString>(3)?.value(&mut context);

	vec_to_array(
		context,
		dialogs::open_file_dialog_multi(
			title,
			default_value,
			Some((&filter_patterns, filter_description)),
		)
		.unwrap_or(vec![]),
	)
}

fn folder(mut context: FunctionContext) -> JsResult<JsString> {
	let title = &*context.argument::<JsString>(0)?.value(&mut context);
	let default_value = &*context.argument::<JsString>(1)?.value(&mut context);

	Ok(context
		.string(dialogs::select_folder_dialog(title, default_value).unwrap_or("".to_string())))
}

register_module!(mut module, {
	module.export_function("color", color)?;
	module.export_function("alert", alert)?;
	module.export_function("saveFile", save_file)?;
	module.export_function("saveFileFilter", save_file_filter)?;
	module.export_function("openFile", open_file)?;
	module.export_function("openFileFilter", open_file_filter)?;
	module.export_function("openFileMultiple", open_file_multiple)?;
	module.export_function("openFileMultipleFilter", open_file_multiple_filter)?;
	module.export_function("folder", folder)?;
	Ok(())
});
