[install-node]: https://nodejs.org/en/download/
[install-holochain]: https://developer.holochain.org/start.html

# Installation & Usage

* **Note 1**: You'll need to [install Node JS (10 or above)][install-node]

* **Note 2**: You'll need to [Install the Holochain command line dev tool][install-holochain]

1. Clone this repo and access the project folder:
```shell
    git clone https://github.com/rzylber/holochainGQLContainerTest.git && cd holochainGQLContainerTest
```

2. Install de dependencies
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

[TODO:]

setup.json

schema.js

npm start or npm run dev