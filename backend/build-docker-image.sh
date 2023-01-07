#!/bin/bash
# build docker container locally (for testing)

docker build -f $PWD/backend/Dockerfile -t controller-tools-backend .