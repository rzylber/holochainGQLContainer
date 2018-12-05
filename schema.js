const { gql } = require('apollo-server-express');

module.exports = ( hApps ) => {
  
  // Type definitions
  const typeDefs = gql`
    type Query {
      getPosts: [Post]
      getPost( address: String! ): Post
      showGreeting( name: String ): String
    }
    type Mutation {
      addPost( text: String! ): String
    }
    type Post {
      text: String
    }
  `;

  // Resolvers
  const resolvers = {
    Query: {
      getPosts: () => {
        return hApps['hApp1'].call("blog", "main", "get_posts", {})
      },
      getPost: (_, { address }) => {
        return hApps['hApp1'].call("blog", "main", "get_post", { address })
      },
      showGreeting: (_, { name }) => {
        return hApps['hApp2'].call("foo", "main", "test", { name }).greeting
      },      
    },
    Mutation: {
      addPost: (_, { text }) => {
        return hApps['hApp1'].call("blog", "main", "send_post", { text }).address 
      }
    }
  };

  return { typeDefs, resolvers }
}