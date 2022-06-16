#!/bin/bash

# creates docker image with production build

PROJECT_NAME=sysmon_bot
DOCKER_IMAGE=$PROJECT_NAME

# go to project root
cd $(dirname $(realpath "$0")) && cd ../..

# create docker image
docker build . --file ./docker/Dockerfile --tag $DOCKER_IMAGE