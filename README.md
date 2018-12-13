[install-node]: https://nodejs.org/en/download/
[install-holochain]: https://developer.holochain.org/start.html

# Overview

> Point to any hApp (bundle) you've created and build schemas (typeDefs and resolvers) over their zome functions.

**Note**: 
This project is a simple application for learning purposes.
This application implements a graphql layer over a node.js holochain container.


# Installation & Usage
If you want to jump to the examples, [*see the sample application*](#sample-application) below

1. First, install all the dependencies:
``` shell
  npm i -S git+https://git@github.com/rzylber/holochainGQLContainer.git apollo-server-express graphql
```

2. Create a `setup.json` file pointing to the hApps you want to instantiate:

``` javascript
{
    "port": 4141, // http and graphQL server port
    "happs": { // hApps to instantiate as "user": "hApp bundle path"
        "movies": "./hApps/movies/dist/bundle.json",
        "hApp2": "./hApps/hApp2/dist/bundle.json"
    }
}
```

3. Create a `schema.js` file with graphQL typeDefs and resolvers. See an example:

``` javascript
const { gql } = require('apollo-server-express');

module.exports = ( hApps ) => {
  
  // Type definitions
  const typeDefs = gql`
    type Query {
      getPeople: [Person]
      getPerson( address: String! ): Person
    }
    type Mutation {
      addPerson( name: String!, gender: String!, place_birth: String! ): String
    }
    type Person {
      address: String,
      name: String
      gender: String
      place_birth: String
    }
  `;

  // Resolvers
  const resolvers = {
    Query: {
      getPeople: () => {
        return hApps['movies'].call("graph", "main", "get_people", {})
      },
      getPerson: (_, { address }) => {
        return hApps['movies'].call("graph", "main", "get_person", { person_address: address })
      },      
    },
    Mutation: {
      addPerson: (_, { name, gender, place_birth }) => {
        return hApps['movies'].call("graph", "main", "create_person", { name, gender, place_birth }).address 
      }
    }
  };

  return { typeDefs, resolvers }
}
```

4. Create your `index.js` following the example below:

``` javascript
const holoGQLContainer = require('holoGQLContainer')

const schema = require('./schema');
const setup = require('./setup.json');

(async ()=>{
    const { server, hApps } = await holoGQLContainer( schema, setup )

    console.log(`🚀 Server ready at http://localhost:${setup.port}${server.graphqlPath}`)
})()
```
5. Run your application: `node index`

6. Open the browser at this url: http://localhost:4141/graphql

See some [*GraphQL query samples*](#graphQL-query-samples) below

# Sample application

* **Note 1**: You'll need to [install Node JS (10 or above)][install-node]

* **Note 2**: You'll need to [Install the Holochain command line dev tool][install-holochain]

1. Clone this repo and access the project folder:
```shell
    git clone https://github.com/rzylber/holochainGQLContainer.git && cd holochainGQLContainer
```

2. Install the dependencies
``` shell
npm install
```

3. Build and pack the test hApps

``` shell
npm run hc-package
```

4. Run the application

``` shell
npm start
```

5. Open the browser at this url: http://localhost:4141/graphql

# GraphQL Query Samples

In the graphql playground, try out some queries and mutations (some sandbox data was inserted for testing purpuse - examples below)
```graphql
query getMovies {
  getMovies {
    name
    year
    language
    actors {
      name
      gender
    }
    director {
      name
      place_birth
    }
  }
}
```