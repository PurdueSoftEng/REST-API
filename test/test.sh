#!/bin/bash

curl -s http://localhost:9090/get
res=$?
until test "$res" == "0"; do
   curl http://localhost:9090/get
   res=$?
done

curl -X GET http://localhost:9090/get

curl -X POST http://localhost:9090/post

curl -X DELETE http://localhost:9090/delete/1

curl -X GET http://localhost:9090/shutdown
