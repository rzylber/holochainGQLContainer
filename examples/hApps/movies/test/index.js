// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

const person1 = {
  name: "Harrison Ford",
  gender: "male",
  place_birth: "Chicago, Illinois, USA"
}
const person1Address = "QmanjpYN3ypTPK7mGE7NWN7EQ7jpjg7FyqJve3r8Rwsxdf";

const person2 = {
  name: "Ridley Scott ",
  gender: "male",
  place_birth: "South Shields, County Durham, England, UK"
}
const person2Address = "QmeJxuwd8VsVPo8couVGjDCnheYH35hN4wZHSBdLwoh5KG";

const movie = {
  name: "Blade Runner",
  year: "1982",
  language: "English"
}

const movieAddress = "Qmct92JbX7DhUNdbjPDSJs918iwkL1YeG6cjJDzkP2b1zT";

test('create person 1', (t) => {
  const result = app.call("graph", "main", "create_person", person1)
  t.equal(result.address, person1Address)
  t.end()
})

test('create movie', (t) => {
  const result = app.call("graph", "main", "create_movie", movie)
  t.equal(result.address, movieAddress)
  t.end()
})

test('create person 2', (t) => {
  const result = app.call("graph", "main", "create_person", person2)
  t.equal(result.address, person2Address)
  t.end()
})

test('get people', (t) => {
  const result = app.call("graph", "main", "get_people", {})
  t.equal(result.length, 2)
  t.end()
})

test('get movies', (t) => {
  const result = app.call("graph", "main", "get_movies", {})
  t.equal(result.length, 1)
  t.end()
})

test('add person1 as actor to movie', (t) => {
  const result = app.call("graph", "main", "add_actor", { actor_address: person1Address, movie_address: movieAddress })
  t.equal(result.success, true)
  t.end()
})

test('add person2 as director to movie', (t) => {
  const result = app.call("graph", "main", "add_director", { director_address: person2Address, movie_address: movieAddress })
  t.equal(result.success, true)
  t.end()
})

test('get movies by actor (person1)', (t) => {
  const result = app.call("graph", "main", "get_movies_by_actor", { actor_address: person1Address })
  t.equal(result[0].name === movie.name, true)
  t.end()
})

test('get actors by movie', (t) => {
  const result = app.call("graph", "main", "get_actors_by_movie", { movie_address: movieAddress })
  t.equal(result[0].name === person1.name, true)
  t.end()
})

test('get movies by director (person2)', (t) => {
  const result = app.call("graph", "main", "get_movies_by_director", { director_address: person2Address })
  t.equal(result[0].name === movie.name, true)
  t.end()
})

test('get director by movie', (t) => {
  const result = app.call("graph", "main", "get_director_by_movie", { movie_address: movieAddress })
  t.equal(result[0].name === person2.name, true)
  t.end()
})