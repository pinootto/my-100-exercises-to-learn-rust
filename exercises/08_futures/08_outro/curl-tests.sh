#!/bin/bash

curl -H 'Content-Type: application/json' -d '{"title":"foo","description":"bar"}' -X POST localhost:4000/ticket

curl -H 'Content-Type: application/json' -d '{"title":"fee","description":"baz"}' -X POST localhost:4000/ticket

curl -H 'Content-Type: application/json' -d '{"title":"FULL","description":"BANG"}' -X PATCH localhost:4000/ticket/0

curl -H 'Accept: application/json' localhost:4000/ticket/0

curl -H 'Accept: application/json' localhost:4000/ticket/1

curl -H 'Accept: application/json' localhost:4000/
