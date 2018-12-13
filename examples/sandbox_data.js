module.exports = ( hApps ) => {

    hApps['movies'].call("graph", "main", "create_person", { 
        name: "Harrison Ford", 
        gender: "male", 
        place_birth: "Chicago, Illinois, USA" 
    })

    hApps['movies'].call("graph", "main", "create_person", { 
        name: "Ridley Scott", 
        gender: "male", 
        place_birth: "South Shields, County Durham, England, UK" 
    })

    hApps['movies'].call("graph", "main", "create_movie", { 
        name: "Blade Runner", 
        year: "1982", 
        language: "English"
    })

    hApps['movies'].call("graph", "main", "add_actor", { 
        actor_address: "QmcMWCXtxzM2TCkom2dqZHPz61njgsyUKM7CNMbVjC8TZr" , 
        movie_address: "QmUhEPoeRiC6KFc5wDHPymisyMdcoGv7vSBYsmdpqB9qBk" 
    })

    hApps['movies'].call("graph", "main", "add_director", {
        director_address: "QmRzdV7vaq8hZghRi46eKXZcK5ijz7zj17hwGpJ666XQHs" , 
        movie_address: "QmUhEPoeRiC6KFc5wDHPymisyMdcoGv7vSBYsmdpqB9qBk" 
    })
}