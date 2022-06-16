#!/bin/bash

# pushes docker image to Docker Hub

PROJECT_NAME=sysmon_bot
DOCKER_IMAGE=$PROJECT_NAME
HUB_USER_NAME=artinphares
HUB_DOCKER_IMAGE=$HUB_USER_NAME/$DOCKER_IMAGE

# push
docker tag $DOCKER_IMAGE $HUB_DOCKER_IMAGE
docker push $HUB_DOCKER_IMAGE