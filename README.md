# Roarich, the Independent Importer

The goal of this project is to import and process locally-stored character
data for Final Fantasy XIV.

Right now the only thing it does is load a GEARSET.DAT, which can then in
turn be exported into [xivgear.app](https://xivgear.app/).

## Notes

- still have a lot of UI work to try and get through, this is mostly proof-of-concept
- are other gearset websites supportable
- web build is probably very broken right now
    - backend data comes via ironworks and expects game data available locally
    - would need to add a xivapi backend for web
