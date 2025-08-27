# Releasing Blinksy

## Pre-requisities

- `just`

## Release new version

Manually update all the crate versions, including internal dependencies.

Commit with the version, e.g. `v0.6.0`.

Then tag and push each crate, starting with `blinksy`:


```shell
just tag {{crate}}
```

Where

- `{{crate}}` is the name of the crate, e.g. `blinksy`.

Then our GitHub Action will build a new release on GitHub.

Once a crate is released, then tag and push the crate's dependents.
