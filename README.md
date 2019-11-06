# Gitlab hook test environment

This repository contains the testing infrastructure for testing gitlab webhook receivers.

In order to test webhook receivers, the code spins up a new gitlab instance using `docker`, along with a runner (to test the pipeline messages) and the webhook defined under `src`, written in Rust.

## Gitlab setup

Start by running the main gitlab server

```
docker-compose up -d gitlab
```

Once it settles down, log in on `http://localhost:8080` and create an admin password.

## Runner setup

Start the runner with

```
docker-compose up -d runner
```

This will fail initially (as it is not configured), so exec into the container and run the [gitlab runner setup](https://docs.gitlab.com/runner/register/):

```
docker-compose exec runner bash
gitlab-runner register
```

Enter the following options:

* Coordinator URL: http://gitlab
* Token: find the shared runner token in the admin pages
* Description: <anything>
* Tags: <anything>
* Executor: docker
* Image: alpine:latest

This will generate a `config.toml` file under `./gitlab/runner/config/config.toml`.

## Hook setup

Build the container and run:

```
docker-compose build
docker-compose up -d hook
```

Set up a project to push to, and configure the webhook under `integrations`:

* url: http://hook
* disable SSL verification

Events passed to the webhook will be written to `hook/requests.txt`

Finally push to the project, and events will be generated.
