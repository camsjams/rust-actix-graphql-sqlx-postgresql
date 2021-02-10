#! /bin/bash
# Run some container

docker run \
	--name postgres \
	-e POSTGRES_PASSWORD=password \
	-p 5432:5432 \
	-v my.conf:/etc/mysql/conf.d \
	-d postgres:alpine
