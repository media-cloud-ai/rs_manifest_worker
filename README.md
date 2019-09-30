# Rust MANIFEST worker


## Requirements

The following tool must be installed on your computer:

* Rust development environment (see installation [here](https://www.rust-lang.org/learn/get-started))
	* Rust >= 1.36.0
	* Cargo  >= 1.36.0
* Rust CI tools:
	* Tarpaulin (Code coverage for unit tests) >= 0.8.4 / see installation [here](https://github.com/xd009642/tarpaulin)
	* Rustfmt (code format) => 1.2.2-stable / see installation [here](https://github.com/rust-lang/rustfmt)
	* Clippy (Rust linter) >= 0.0.212 / see installation [here](https://github.com/rust-lang/rust-clippy)
* JQ (see installation [here](https://stedolan.github.io/jq/download/))

## Launch worker locally

Before to launch the worker you need to set some environment variables. These variables are describe [here](https://github.com/media-cloud-ai/rs_amqp_worker).
)

One variable could be set specifically for this worker: 

| Variable name          | Default value | Possible values | Description                                 |
|------------------------|---------------|-----------------|---------------------------------------------|
| `MANIFEST_MODE`        | `DASH`        | `ISM` or `DASH` | (Case sensitive) If value is `DASH` (or if variable is not set) the worker will process dash format. Otherwise, it will process ism format. |


Once these environment variables are set, you can start your worker:
```bash
make run
```

### Trick to set environment variables easily

You could create a file named `.env` (or you can copy the file `.env.dist`) end edit it with the correct values.
The `make run` command will automatically take it into account if it exists.

## Makefile targets

Commands below will be used for both stacks (backend & workers):

| Command                     | Description                                              |
|-----------------------------|----------------------------------------------------------|
| `make build`                | Build the application                                    |
| `make ci-code-format`       | Check the code format according to the rust format rules |
| `make ci-code-coverage`     | Launch tests and returns code coverage for tests         |
| `make ci-lint`              | Launch the rust linter                                   |
| `make ci-tests`             | Launch tests                                             |
| `make docker-build`         | Build locally a docker image                             |
| `make docker-clean`         | Remove locally the built docker image                    |
| `make docker-push-registry` | Push the locally built docker image                      |
| `make run`                  | Run locally the worker                                   |
| `make version`              | Display the version defined in `cargo.toml` file         |

## CI / CD

A `.gitlab-ci.yml` file is provided for the gitlab CI/CD feature.
This file will instantiate te following pipeline:

<!-- language: lang-none -->
    /-----------\      /---------\             /----------\             /----------\
    |  Compile  |------|  Tests  |-------------| Quality  |-------------|  Docker  |
    \-----------/      \---------/             \----------/             \----------/
         |                  |                       |                        |
     +------+           +-------+              +---------------+          +-------+
     | lint |           | tests |              | code-coverage |          | build |
     +------+           +-------+              +---------------+          +-------+
                                                    |
                                               +-------------+
                                               | code-format |
                                               +-------------+
<!-- language: markdown -->

### Docker

The command `make docker-build` will build an image named `mediacloudai/manifest_worker`.

The command `make push-docker-registry` will logged in and push the built image in the official docker registry. The login must be set with the following environment variables:

| Variable name           | Default value              | Description                                      |
|-------------------------|----------------------------|--------------------------------------------------|
| `DOCKER_REGISTRY_LOGIN` |                            | User name used to connect to the docker registry |
| `DOCKER_REGISTRY_PWD`   |                            | Password used to connect to the docker registry  |
| `RUSTTOOLS_DOCKER_IMG`  |                            | Name of RUST image containing CLIPPY, TARPAULIN and RUSTFMT. |
