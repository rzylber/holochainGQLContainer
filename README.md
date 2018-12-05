[install-node]: https://nodejs.org/en/download/
[install-holochain]: https://developer.holochain.org/start.html

# Overview

This project is a simple application for learning purposes.
This application implements a graphql layer over a node.js holochain container.

**Caution:** Holochain is in Alpha 2 version.

# Installation & Usage

* **Note 1**: You'll need to [install Node JS (10 or above)][install-node]

* **Note 2**: You'll need to [Install the Holochain command line dev tool][install-holochain]

1. Clone this repo and access the project folder:
```shell
    git clone https://github.com/rzylber/holochainGQLContainerTest.git && cd holochainGQLContainerTest
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

6. In the graphql playground, try out some queries and mutations (examples below)
```graphql
query getPosts{
  getPosts {
    text
  }
}

mutation addMyPost {
  addPost( text: "My post" )
}

mutation addAnotherPost {
  addPost( text: "Another post" )
}

query getMyPost {
  getPost( address: "QmdzsvNqxEkExTvWnbn1HJLqXGKNYyo5D9goz3P4M5b5YR" ) {
    text
  }
}

query getAnotherPost {
  getPost( address: "QmZNoZhFgXRv8bjdmona9J7bQFwiJhB3TVCsEPthLRKAnL" ) {
    text
  }
}

query showGreeting {
  showGreeting( name: "Zaphod Beeblebrox" )
}
```

# Custom hApps and GraphQL schemas

You can point any hApp (bundle) you've created and create any new schema (typeDefs and resolvers), changing the following files:

1. Edit `setup.json` and point to the hApps you want to instantiate:

``` javascript
{
    "port": 4141, // http and graphQL server port
    "happs": { // hApps to instantiate as "user": "hApp bundle path"
        "hApp1": "./hApp/hApp1/dist/bundle.json",
        "hApp2": "./hApp/hApp2/dist/bundle.json"
    }
}
```

2. Edit `schema.js` following its patterns to create graphQL typeDefs and resolvers.

3. And then, you can start the server and access the playground with the following command:
``` shell
npm start
```
alternatively you can run the command below for development (nodemon):

``` shell
npm run dev
```