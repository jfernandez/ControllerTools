#!/bin/bash
# build docker container locally (for testing)

docker build -f $PWD/Dockerfile -t controller-tools-backend .