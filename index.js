const holoGQLContainer = require('./holoGQLContainer')

const schema = require('./schema');
const setup = require('./setup.json');

(async ()=>{
    const { app, server } = await holoGQLContainer( schema, setup )

    console.log(`🚀 Server ready at http://localhost:${setup.port}${server.graphqlPath}`)
})()