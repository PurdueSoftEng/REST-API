#!/bin/bash

curl -s http://localhost:5000/get
res=$?
until test "$res" == "0"; do
   curl -s http://localhost:5000/get
   res=$?
done

curl -X GET http://localhost:5000/get

curl -X POST http://localhost:5000/post

curl -X DELETE http://localhost:5000/delete/1
