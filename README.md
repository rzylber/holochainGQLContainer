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