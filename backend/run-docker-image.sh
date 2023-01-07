#!/bin/bash

docker run -i --entrypoint /backend/entrypoint.sh -v $PWD/backend:/backend controller-tools-backend