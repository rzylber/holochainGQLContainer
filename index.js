const express = require('express');
const { ApolloServer } = require('apollo-server-express');
const Container = require('@holochain/holochain-nodejs');

module.exports = async ( schema, setup ) => {

  // instantiate and start hApps
  const hApps = {}
  for( key in setup.happs ) {
    hApps[key] = Container.instanceFromNameAndDna(key, setup.happs[key] )
    hApps[key].start()    
  }

  const server = new ApolloServer( schema(hApps) );

  const app = express();
  server.applyMiddleware({ app });
  
  await app.listen({ port: setup.port || 4400 });
  return { app, server, hApps };
}