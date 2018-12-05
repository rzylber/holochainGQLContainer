// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

const text = "My first post"
const address = "QmcxVuotGN8Gx1VFhHXdTJPNDoek1rguaaeT6veFsv8Paa"

test('get posts (start)', (t) => {
  const posts = app.call("blog", "main", "get_posts", {})
  t.equal(posts.length, 0)
  t.end()
})

test('save post', (t) => {
  const result = app.call("blog", "main", "send_post", { text })
  t.equal(result.address, address)
  t.end()
})

test('get post', (t) => {
  const post = app.call("blog", "main", "get_post", { address })
  t.equal(post.text, text)
  t.end()
})

test('get posts (end)', (t) => {
  const posts = app.call("blog", "main", "get_posts", {})
  t.equal(posts[0].text, text)
  t.end()
})
