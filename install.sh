#!/bin/sh
docker build -f docker/Dockerfile.front -t frontend:latest .
#docker build -f docker/Dockerfile.exonum -t exonum:latest .
docker build -f docker/Dockerfile.node -t node:latest .

docker-compose -f docker/docker-compose-exonum.yml stop
docker-compose -f docker/docker-compose-exonum.yml rm -f
docker-compose -f docker/docker-compose-exonum.yml pull
docker-compose -f docker/docker-compose-exonum.yml up -d

docker-compose -f docker/docker-compose-node.yml stop
docker-compose -f docker/docker-compose-node.yml rm -f
docker-compose -f docker/docker-compose-node.yml up -d

docker-compose -f docker/docker-compose-storages.yml stop
docker-compose -f docker/docker-compose-storages.yml rm -f
docker-compose -f docker/docker-compose-storages.yml up -d

docker-compose -f docker/docker-compose-frontend.yml stop
docker-compose -f docker/docker-compose-frontend.yml rm -f
docker-compose -f docker/docker-compose-frontend.yml up -d
