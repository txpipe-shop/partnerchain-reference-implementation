# Partner Chains Local Environment

This stack is designed to run a local development testnet Cardano node complete with Ogmios, PostgreSQL and Db-sync services. This is the stack that a Partner chain node needs to connect to a Mainchain.

This environment is based on IOG's Partner Chain [local environment](https://github.com/input-output-hk/partner-chains/tree/master/dev/local-environment), and has been modified to fit our needs. We don't support the stack that sets up partner chain nodes too but the configuration files are kept and these can be used from outside the docker.  

The local environment includes:

- 1 x Cardano Node running private testnet with pre-configured genesis files (2 minutes epochs)
- 1 x PostgreSQL database
- 1 x Db-sync
- 1 x Ogmios

The stack `setup.sh` script will create a docker-compose.yml stack configuration files, and populate an .env file with environment values. The stack can be deployed with `docker-compose up -d`, and the services should be stopped with `docker compose down --volumes`.

## Local env - step by step

- When first run, all images will be pulled from public repositories. This stage may take some time. The stack will then be built and run.
- When the stack is running, the Cardano node begins block production. This is a private testnet and will not connect to the public Cardano network, but rather from a pre-configured genesis file.
- Once the Cardano chain is synced, Ogmios and DB-Sync will in turn connect to the Cardano node node.socket and begin syncing the chain.

## Initialising the environment configuration

Run `setup.sh` script to enter the setup wizard for initialising the environment .env values and docker-compose.yml. The `setup.sh` script also support a `--non-interactive` flag to accept default configuration settings, or configuration elements can be specified directly with args (see `--help` for details)

```
chmod +x setup.sh
bash setup.sh`
```

## Starting the environment

Once initialized, deploy the local environment from .env values with the following:

```
docker compose up -d
```

We recommend using a visual Docker UI tool such as [lazydocker](https://github.com/jesseduffield/lazydocker) or [Docker Desktop](https://www.docker.com/products/docker-desktop/) for following the live logs and performance of all containers in the environment. Each component has been scripted to provide verbose logging of all configuration actions it is performing to demonstrate the end-to-end setup of a Cardano Partner Chain.

## Stopping the environment

When stopping the stack, it is mandatory to also wipe all volumes. The environment does not yet support persistent state. To tear down the environment and remove all volumes, use the following:

```
docker compose down --volumes
```

## Other Features

We are continuing to develop this utility for a range of applications, and regularly add additional features. The `setup.sh --help` output will always show the latest details for available features:

```
$ bash setup.sh --help
Usage: setup.sh [OPTION]...
Initialize and configure the Docker environment.
  -n, --non-interactive     Run with no interactive prompts and accept sensible default configuration settings.
  -p, --postgres-password   Set a specific password for PostgreSQL (overrides automatic generation).
  -i, --node-image          Specify a custom Partner Chains Node image.
  -t, --tests               Include tests container.
  -h, --help                Display this help dialogue and exit.
```
