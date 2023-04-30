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

curl -X POST -H "Content-Type: application/json" -d '{"url":"https://github.com/jashkenas/underscore", "version":"1.2.3", "package_name":"underscore", "jsprogram":"program.exe", "content":"base64string", "metric_one":0, "metric_two":0, "metric_three":0, "metric_four":0, "metric_five":0, "metric_six":0, "metric_seven":0, "total_score":0}' https://tool-app-6x33ng5huq-uc.a.run.app/package