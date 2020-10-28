// @ts-expect-error Native module
import native from "../native" // eslint-disable-line import/no-unresolved

/**
Add an option in the dialog to only show files that match a specific pattern.
*/
type Filter = {
	/**
	An array of patterns which shown files must match. Otherwise they are hidden when the filter is selected. A `*` can be used as a wildcard. For example: `*.txt`.
	*/
	patterns: string[]

	/**
	The description of the filter to display. For example: `Text Files (*.txt)`.
	*/
	description: string
}

/**
Ask the user to select a color.
*/
export const color = ({
	title = "Choose a color",
	defaultValue = "#000000"
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The default selected color of the dialog.
	*/
	defaultValue?: string
} = {}): string => {
	if (defaultValue.length === 6) {
		defaultValue = `#${defaultValue}`
	}

	return native.color(title, defaultValue)
}

/**
Show an alert box.
*/
export const alert = ({
	title = "Info",
	message = "",
	defaultValue = true,
	icon = "info",
	buttons = "ok"
}: {
	/**
	The title of the alert.
	*/
	title?: string

	/**
	The message of the alert.
	*/
	message?: string

	/**
	The default value of the dialog if no action is selected. The right dialog button has a value of `true` and the left dialog button has a value of `false`.
	*/
	defaultValue?: boolean

	/**
	The icon to use in the dialog.
	*/
	icon?: "info" | "warning" | "error" | "question"

	/**
	The buttons to show in the dialog.
	*/
	buttons?: "yesno" | "okcancel" | "ok"
} = {}): boolean => {
	if (!["yesno", "okcancel", "ok"].includes(buttons)) {
		throw new TypeError("`buttons` must be `yesno`, `okcancel` or `ok`")
	}

	if (!["info", "warning", "error", "question"].includes(icon)) {
		throw new TypeError("`icon` must be `info`, `warning`, `error` or `question`")
	}

	return native.alert(title, message, icon, buttons, defaultValue)
}

/**
Ask the user to choose a path to save a file in.
*/
export const saveFile = ({
	title = "Save file",
	defaultValue = "",
	filter
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The default path to set.
	*/
	defaultValue?: string

	filter?: Filter
} = {}): string | undefined => {
	if (filter) {
		return native.saveFileFilter(title, defaultValue, filter.patterns, filter.description) || undefined
	}

	return native.saveFile(title, defaultValue) || undefined
}

/**
Ask the user to select a file.
*/
export const openFile = ({
	title = "Select file",
	defaultValue = "",
	filter
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The default path to set.
	*/
	defaultValue?: string

	filter?: Filter
} = {}): string | undefined => {
	if (filter) {
		return native.openFileFilter(title, defaultValue, filter.patterns, filter.description) || undefined
	}

	return native.openFile(title, defaultValue) || undefined
}

/**
Ask the user to select 1 or more files.
*/
openFile.multiple = ({
	title = "Select files",
	defaultValue = "",
	filter
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The default path to set.
	*/
	defaultValue?: string

	filter?: Filter
} = {}): string[] => {
	if (filter) {
		return native.openFileMultipleFilter(title, defaultValue, filter.patterns, filter.description)
	}

	return native.openFileMultiple(title, defaultValue)
}

/**
Ask the user to select a folder.
*/
export const folder = ({
	title = "Select folder",
	defaultValue = ""
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The default path to set.
	*/
	defaultValue?: string
} = {}): string | undefined => {
	return native.folder(title, defaultValue) || undefined
}

/**
Ask the user for text input.
*/
export const input = ({
	title = "Input",
	message = "",
	defaultValue = "",
	isPassword = false
}: {
	/**
	The title of the dialog.
	*/
	title?: string

	/**
	The message of the prompt.
	*/
	message?: string

	/**
	The default value to prefill in the dialog. Doesn't work if `isPassword` is true.
	*/
	defaultValue?: string

	/**
	Whether the input should be treated as a password. If set to `true`, the typed text will appear as dots.
	*/
	isPassword?: boolean
} = {}): string => {
	if (isPassword) {
		return native.passwordInput(title, message)
	}

	return native.textInput(title, message, defaultValue)
}
