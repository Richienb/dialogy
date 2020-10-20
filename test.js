const test = require("ava")
const dialogy = require(".")

test("main", t => {
	t.is(typeof dialogy, "object")
})
