# OAuth2 server for wrapping Shibboleth IdPs

**This is currently missing documentation**

This is a server that creates an OAuth2 Server (identity provider).
It assumes it runs behind as protected resource (e.g. by securing it with `AuthType shibboleth` in an Apache 2 server). 
Shibboleth/SAML meta data fields that are passed through as HTTP headers (like `X-Remote-User`) variables can be mapped to JWT token attributes.

## TODOs

- [X] Basic implementation of the OAuth2-Workflow using the [oxide-auth](https://github.com/HeroicKatora/oxide-auth/) library
- [X] Support configuration files for the various settings
- [X] Generate JWT tokens as bearer tokens
- [X] Allow to define mappings between HTTP headers and generated JWT tokens
- [ ] Document integration with the Apache2 Shibboleth module and how to configure and start this service
- [ ] Add release binaries for Linux

## Background

This project will be used as identity provider for the [ANNIS frontend](https://github.com/korpling/ANNIS) when an institutional Shibboleth identity provider (like the DFN AAI) should be used.
