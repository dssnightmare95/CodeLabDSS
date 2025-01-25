const express = require('express');
const router = express.Router();
const passport = require('passport');
const axios = require('axios');

require('../auth')(passport);

const teamsController = require('../controllers/teams');
const { getUser } = require('../controllers/users');

router.route('/')
    .get( passport.authenticate('jwt', {session: false}), 
            (req, res) => {
                let user = getUser(req.user.userId);
                res.status(200).json({
                    trainer: user.userName,
                    team: teamsController.getTeamOfUser(req.user.userId)
                })
        })
    .put( passport.authenticate('jwt', {session: false}),
        (req, res) => {
        teamsController.setTeam(req.user.userId, req.body.team);
        res.status(200).send();
    })

router.route('/pokemons')
    .post(passport.authenticate('jwt', {session: false}), (req, res) => {
        let pokemonName = req.body.name;
        axios.get(`https://pokeapi.co/api/v2/pokemon/${pokemonName.toLowerCase()}`)
            .then((response) => {
                teamsController.addPokemon(req.user.userId, pokemonName, response.data.id);
                res.status(201).json({name: pokemonName, pokedexNumber: response.data.id});
            })
            .catch((error) => {
                res.status(400).json({message: error});
            });
    })

router.route('/pokemons/:pokeid')
    .delete(passport.authenticate('jwt', { session: false }), (req, res) => {
        const pokeid = req.params.pokeid;
        teamsController.removePokemon(req.user.userId, pokeid);
        let team = teamsController.getTeamOfUser(req.user.userId);
        let user = getUser(req.user.userId);
        res.status(200).json({trainer: user.userName, team: team});
    });
    
exports.router = router;