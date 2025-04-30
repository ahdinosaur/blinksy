# Releasing Blinksy

## Pre-requisities

- `just`

## Release new version

```shell
just release {{crate}} {{bump}}
```

Where

- `{{crate}}` is the name of the crate, e.g. `blinksy`.
- `{{bump}}` is the version change, i.e. `patch`, `minor`, `major`, or a specific version.

Then our GitHub Action will build a new release on GitHub.
