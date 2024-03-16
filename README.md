# Typescript CRUD Generator

CLI to generate CRUD files for Typescript projects.

## Quick start

```bash
brew install rust # install rust
git cloneithub.com:degu-io/ts-crud_generator.git # clone project
cd ts-crud_generator/
cargo run -- -e User
```

This will create a `build/` directory with the a typscript file containing a basic TRPC CRUD router.

## TODO

- [ ] Add Drizzle schema
- [ ] Add more CRUD files (currently only provides a TRPC router)
- [ ] Add type of project CRUD selector
