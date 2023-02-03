# Composer Update Autocomplete for Bash

This small tool allows using Bash autocompletion (with `<TAB><TAB>`) when updating Composer modules.

It first tries to find the `composer.json` file on the current working directory, then searches each non-hidden child directory (in parallel) until it finds one. It does **not** recursively search the child directories.

## Why

I was tired of typing very long and easy to misspell module names at work.

## How (to use)

- Download the binary
- Add any of the following lines to your `.bashrc` file:
```bash
complete -C /path/to/binary docker-compose run --rm composer update # add this if you (like me) run composer inside a docker container
complete -C /path/to/binary docker-compose run composer update # add this in case you forget to do --rm
complete -C /path/to/binary composer update # add this if you use a composer installation (or in my case, an alias for the above)
```
