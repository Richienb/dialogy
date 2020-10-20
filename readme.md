# dialogy [![Travis CI Build Status](https://img.shields.io/github/workflow/status/Richienb/dialogy/Test/master?style=for-the-badge)](https://github.com/Richienb/dialogy/actions?query=branch%3Amaster)

Show native dialogs.

[![NPM Badge](https://nodei.co/npm/dialogy.png)](https://npmjs.com/package/dialogy)

## Install

```sh
npm install dialogy
```

## Usage

```js
const dialogy = require("dialogy")

dialogy.alert({ message: "Hello World" })

const filePath = dialogy.openFile({
	filter: {
		patterns: ["*.txt"],
		description: "Text files"
	}
})
```

## API

### dialogy.color(options?)

Ask the user to select a color.

#### options

Type: `object`

##### title

Type: `string`\
Default: `Choose a color`

The title of the dialog.

##### defaultValue

Type: `string`\
Default: `#000000`

The default selected color of the dialog.

### dialogy.alert(options?)

Show an alert box.

#### options

Type: `object`

##### title

Type: `string`\
Default: `Info`

The title of the alert.

##### message

Type: `string`

The message of the alert.

##### defaultValue

Type: `boolean`\
Default: `true`

The default value of the dialog if no action is selected. The right dialog button has a value of `true` and the left dialog button has a value of `false`.

##### icon

Type: `string`\
Values: `info | warning | error | question`

The icon to use in the dialog.

##### buttons

The buttons to show in the dialog.

### dialogy.saveFile(options?)

Ask the user to choose a path to save a file in.

#### options

Type: `object`

##### title

Type: `string`\
Default: `Save file`

The title of the dialog.

##### defaultValue

Type: `string`

The default path to set.

##### filter.patterns

Type: `string[]`

An array of patterns which shown files must match. Otherwise they are hidden when the filter is selected. A `*` can be used as a wildcard. For example: `*.txt`.

##### filter.description

Type: `string`

The description of the filter to display. For example: `Text Files (*.txt)`

### dialogy.openFile(options?)

Ask the user to select a file. Returns a string.

Same options as [`dialogy.saveFile`](#dialogysavefileoptions).

### dialogy.openFile.multiple(options?)

Ask the user to select 1 or more files. Returns an array of strings.

Same options as [`dialogy.saveFile`](#dialogysavefileoptions).

### dialogy.folder(options?)

Ask the user to select a folder. Returns a string.

#### options

Type: `object`

##### title

Type: `string`\
Default: `Select folder`

The title of the dialog.

##### defaultValue

Type: `string`

The default path to set.
