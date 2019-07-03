resign-jwt
==========

A tool for re-signing jwts. For example, you may have a development environment that needs a JWT, but does not have
the ability to create them. This tool allows you to re-sign a jwt from another environment with the secret for your
development environment.

Install
-------

```shell
cargo install -f resign-jwt
```

Usage
-----

You can see usage at any time with --help

```shell
resign-jwt --help
```

Typically you'll want to provide three things
- the old `--jwt` (`-j`) (note: you only need to provide the first two parts if you have security concerns)
- the `--algorithm` (`-a`) to use (currently only HSxxx algorithms below, see issues)
- the new `--key` (`-k`) to use for signing

```shell
resign-jwt \
  --jwt eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhlbGxvQGV4YW1wbGUuY29tIn0= \
  --algorithm HS256 \
  --key some-key
```
