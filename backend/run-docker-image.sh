#!/bin/bash

plugin="ControllerTools"
docker_name="controller-tools-backend"

docker run --rm -i -v $PWD/backend:/backend -v /tmp/output/$plugin/backend/out:/backend/out --entrypoint /backend/entrypoint.sh "$docker_name"
mkdir -p /tmp/output/$plugin/bin
cp -r /tmp/output/$plugin/backend/out/. /tmp/output/$plugin/bin
