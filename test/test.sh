#!/bin/bash

curl -s http://localhost:8000/
res=$?
until test "$res" == "0"; do
   curl http://localhost:8000/
   res=$?
done

curl -X POST -H "Content-Type: application/json" -d '{"link": "https://example.com/", "package_name": "JSObject", "metric_one": 0, "metric_two": 1, "metric_three": 0, "metric_four": 2, "metric_five": 1, "metric_six": 0, "metric_seven": 1, "total_score": 5 }' http://localhost:8000/package

curl -X GET -H "Content-Type: application/json" http://localhost:8000/package
curl -X GET -H "Content-Type: application/json" http://localhost:8000/package/Rustic

curl -X PUT -H "Content-Type: application/json" http://localhost:8000/package/Example

curl -X POST -H "Content-Type: application/json" -d '{"group_name": "Group1"}' http://localhost:8000/group

curl -X GET -H "Content-Type: application/json" http://localhost:8000/group