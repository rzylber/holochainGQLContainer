// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

test('description of example test', (t) => {
  const result = app.call("foo", "main", "test", { name: "Ricardo" })
  t.equal(result.greeting, "HI Ricardo")
  t.end()
})
