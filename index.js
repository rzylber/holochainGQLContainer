const holoGQLContainer = require('./holoGQLContainer')

const schema = require('./schema');
const setup = require('./setup.json');

(async ()=>{
    const { server, hApps } = await holoGQLContainer( schema, setup )

    //pre fill with some sandbox data - for testing purposes
    require('./sandbox_data')( hApps )

    console.log(`🚀 Server ready at http://localhost:${setup.port}${server.graphqlPath}`)
})()