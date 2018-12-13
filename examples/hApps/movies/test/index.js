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
const person1Address = "QmcMWCXtxzM2TCkom2dqZHPz61njgsyUKM7CNMbVjC8TZr";

const person2 = {
  name: "Ridley Scott ",
  gender: "male",
  place_birth: "South Shields, County Durham, England, UK"
}
const person2Address = "QmPqhLUFLRChB3kus9ya1HtASXs9vvVq8739vTpqAX99wh";

const movie = {
  name: "Blade Runner",
  year: "1982",
  language: "English"
}

const movieAddress = "QmUhEPoeRiC6KFc5wDHPymisyMdcoGv7vSBYsmdpqB9qBk";

test('create person 1', (t) => {
  const result = app.call("graph", "main", "create_person", person1)
  t.equal(result.address, person1Address)
  t.end()
})

test('get person 1', (t) => {
  const result = app.call("graph", "main", "get_person", { person_address: person1Address })
  t.equal(result.name, person1.name)
  t.end()
})

test('create movie', (t) => {
  const result = app.call("graph", "main", "create_movie", movie)
  t.equal(result.address, movieAddress)
  t.end()
})

test('get movie', (t) => {
  const result = app.call("graph", "main", "get_movie", {movie_address: movieAddress })
  t.equal(result.name, movie.name)
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