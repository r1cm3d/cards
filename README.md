# cards

![Rust version](https://img.shields.io/badge/Rust-1.47.0-lightgrey)
![GitHub](https://img.shields.io/github/license/r1cm3d/cards)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/r1cm3d/cards)
[![GitHub issues](https://img.shields.io/github/issues/r1cm3d/cards?color=green)](https://github.com/ricardomedeirosdacostajunior/aws-poc/issues?q=is%3Aopen+is%3Aissue)
[![GitHub closed issues](https://img.shields.io/github/issues-closed/r1cm3d/cards?color=red)](https://github.com/ricardomedeirosdacostajunior/aws-poc/issues?q=is%3Aissue+is%3Aclosed)
[![Twitter Follow](https://img.shields.io/twitter/follow/r1cm3d?style=social)](https://twitter.com/r1cmed)

**TL;DR:**
```console
make run
```

## Prerequisites
[![Docker](https://img.shields.io/badge/Docker-19.03.9-blue)](https://www.docker.com/)
[![Docker-compose](https://img.shields.io/badge/Docker--compose-1.25.5-blue)](https://github.com/docker/compose/releases)
[![GNU Make](https://img.shields.io/badge/GNU%20Make-4.2.1-lightgrey)](https://www.gnu.org/software/make/)
[![GNU Bash](https://img.shields.io/badge/GNU%20Bash-4.2.1-lightgrey)](https://www.gnu.org/software/bash/)
[![terraform](https://img.shields.io/badge/terraform-0.14.6-blueviolet)](https://github.com/hashicorp/terraform)
[![shfmt](https://img.shields.io/badge/shfmt-v3.1.0-lightgrey)](https://github.com/mvdan/sh)
[![aws-cli](https://img.shields.io/badge/aws--cli-2.0.49-yellow)](https://github.com/aws/aws-cli)

## Table of Contents
* [TL;DR](#aws-poc)
* [Prerequisites](#prerequisites)
* [About the Project](#about-the-project)
* [Getting Started](#getting-started)
* [Testing](#testing)
* [Run](#run)

## About The Project

Web server application that provides a repository for debit and credit cards

## Getting Started

To run this project locally you must have the technologies as the [prerequisites section](#prerequisites)

### Initializing
#### It will create the container and all dependencies
```sh
make init
```

### Linting
```sh
make lint
```

### Formatting
```sh
make fmt
```

### Testing
```sh
make test
```

### Run
#### Build all dependencies and run
```sh
make run
```

### Stopping
#### Stop containers
```sh
make down
```