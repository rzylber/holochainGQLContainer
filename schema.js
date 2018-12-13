const { gql } = require('apollo-server-express');

module.exports = ( hApps ) => {
  
  // Type definitions
  const typeDefs = gql`
    type Query {
      getPeople: [Person]
      getMovies: [Movie]
      getPerson( address: String! ): Person
      getMovie( address: String! ): Movie
      showGreeting( name: String ): String
    }
    type Mutation {
      addPerson( name: String!, gender: String!, place_birth: String! ): String
      addMovie( name: String!, year: String!, language: String! ): String
      addActor( actor_address: String!, movie_address: String! ): Boolean
      addDirector( director_address: String!, movie_address: String! ): Boolean
    }
    type Person {
      address: String,
      name: String
      gender: String
      place_birth: String
    }
    type Movie {
      address: String,
      name: String
      year: String
      language: String
      actors: [Person],
      director: Person
    }
  `;

  // Resolvers
  const resolvers = {
    Query: {
      getPeople: () => {
        return hApps['movies'].call("graph", "main", "get_people", {})
      },
      getMovies: () => {
        return hApps['movies'].call("graph", "main", "get_movies", {})
      },
      getPerson: (_, { address }) => {
        return hApps['movies'].call("graph", "main", "get_person", { person_address: address })
      },
      getMovie: (_, { address }) => {
        return hApps['movies'].call("graph", "main", "get_movie", { movie_address: address })
      },
      showGreeting: (_, { name }) => {
        return hApps['hApp2'].call("foo", "main", "test", { name }).greeting
      },      
    },
    Movie: {
      actors: (movie) => hApps['movies'].call("graph", "main", "get_actors_by_movie", { movie_address: movie.address }),
      director: (movie) => {
        const result = hApps['movies'].call("graph", "main", "get_director_by_movie", { movie_address: movie.address })
        return result.length > 0 ? result[0] : null
      },
    },
    Mutation: {
      addPerson: (_, { name, gender, place_birth }) => {
        return hApps['movies'].call("graph", "main", "create_person", { name, gender, place_birth }).address 
      },
      addMovie: (_, { name, year, language }) => {
        return hApps['movies'].call("graph", "main", "create_movie", { name, year, language }).address 
      },
      addActor: (_, {actor_address, movie_address}) => {
        return hApps['movies'].call("graph", "main", "add_actor", { actor_address, movie_address }).success
      },
      addDirector: (_, {director_address, movie_address}) => {
        return hApps['movies'].call("graph", "main", "add_director", { director_address, movie_address }).success
      }
    }
  };

  return { typeDefs, resolvers }
}