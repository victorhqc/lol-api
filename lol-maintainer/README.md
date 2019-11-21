# lol-maintainer

## Motivation

Keeping up to date with Riot is hard. New seasons come, items get removed, mapps get added. Besides,
must of the code for the API Wrapper is very repetitive and tedious. This CLI Should make it easier
to maintain & write the LOL API Wrapper.

## Scope

This Binary should:

- Fetch League constants & generate models & types.
- Fetch endpoint signatures & attempt to generate types, files, etc.

This binary should not:

- Decide new version
- Warn about breaking changes

Publishing a new lol-api version & update docs should be a manual work, at least in the first
iteration.
