import test, { ExecutionContext } from "ava" // eslint-disable-line @typescript-eslint/no-unused-vars
import { alert } from "./source"

test("main", (t: ExecutionContext) => {
	t.is(typeof alert, "function")
})
