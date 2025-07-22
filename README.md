# Havoc
A CLI tool that makes gRPC gateways easier than ever using a config file (JSON, YAML) (Config as Code)

## Installation

### 1. Tool
Check [releases](https://github.com/Zeann3th/havoc/releases) or clone this repository and run:

```bash
cargo install --path .
```

> [!WARN]
> Make sure you have [Rust](https://www.rust-lang.org/) version >= 1.88 installed!!!

### 2. Config file
Currently supports JSON and YAML

- Example JSON:

```json
{
    "$schema": "https://zeann3th.github.io/havoc/schemas/v0.1/config.json",
    "metadata": {
        "name": "gateway",
        "description": "A simple gateway configuration example",
        "version": "1.0.0",
        "author": "Zeann3th"
    },
    "spec": {
        "host": "127.0.0.1",
        "port": 3000,
        "services": [
            {
                "name": "Auth",
                "proto": "./examples/proto/auth.proto",
                "url": "http://auth-svc.svc.cluster.local:8080",
                "endpoints": [
                    {
                        "rpc": "Login",
                        "method": "POST",
                        "path": "/login",
                        "request": {
                            "type": "LoginRequest",
                            "fields": [
                                {
                                    "name": "username",
                                    "type": "String"
                                },
                                {
                                    "name": "password",
                                    "type": "String"
                                }
                            ]
                        },
                        "response": {
                            "type": "LoginResponse",
                            "fields": [
                                {
                                    "name": "access_token",
                                    "type": "String"
                                },
                                {
                                    "name": "refresh_token",
                                    "type": "String"
                                }
                            ],
                            "cookies": [
                                {
                                    "name": "refresh_token",
                                    "options": {
                                        "httpOnly": true,
                                        "secure": true,
                                        "sameSite": "Strict",
                                        "maxAge": 3600,
                                        "path": "/",
                                        "domain": "example.com",
                                        "partitioned": true
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    }
}
```

- Example YAML:

```yaml
metadata:
  name: gateway
  description: A simple gateway configuration example
  version: "1.0.0"
  author: Zeann3th

spec:
  host: example.com
  port: 80
  services:
    - name: AuthService
      proto: ./examples/proto/auth.proto
      url: http://auth-svc.svc.cluster.local:8080
      endpoints:
        - rpc: Login
          method: POST
          path: /login
          request:
            type: LoginRequest
            fields:
              - name: username
                type: String
              - name: password
                type: String
          response:
            type: LoginResponse
            fields:
              - name: access_token
                type: String
              - name: refresh_token
                type: String
            cookies:
              - name: refresh_token
                options:
                  httpOnly: true
                  secure: true
                  sameSite: Strict
                  maxAge: 3600
                  path: /
                  domain: example.com
                  partitioned: true
```

> [!TIP]
> Please name your services like Auth, Book, Review,... meaning one word... 

### 3. Generating code
To generate code, run:

```bash
havoc new <config-file-path>
```

By default, it will generate code for Rust Axum http Framework, for more frameworks, you can run:

```bash
havoc list-fw

havoc new <config-file-path> -f axum # or nestjs or ...
```

As of 2025/07/22, havoc only support Axum since this is an early build.

Once done, you can add middlewares freely, changing code logic and etc to suit your needs. This tools helps with generating boilerplate code, not a universal solution.

## Road maps
- [ ] Nestjs and other frameworks
- [ ] User defined template files
- [ ] Support Rest, Graphql for config file (currently limited to gRPC)
- [ ] Generate static openapi documentation so users can copy and include in their server

## License
[MIT](./LICENSE)


