#!/bin/bash
declare -A myassoc
myassoc['open the terminal']='gnome-terminal'
myassoc['compose monthly status report']='github-crawler'
myassoc['file an issue to JIRA']='curl'

echo "open the terminal: ${myassoc['open the terminal']}"


issue_template=$'{"fields":{
  "project":{
    "key": "PADDLEQ"},
    "summary": "Test issue (disregard)",
    "description": "PR 27871 introduced a crash to mobilenet_V2 training",
    "issuetype": {
      "name": "Task"}}}'

filename=/tmp/`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 8`
echo $filename

# Put template into temporary file for user to edit
echo "$issue_template" > $filename 
gvim -f $filename 
#Get password from user
pass=`zenity --password`
# Send prepared issue entry to JIRA
result=`curl -D- -u jczaja:$pass -X POST --data @$filename -H Content-Type:application/json https://jira.devtools.intel.com/rest/api/2/issue/`

echo "result: $result"

rm $filename

# Prepare feedback to user
# a) Error on pattern: "errorMessages" 
# b) Success on pattern: "key"
feedback=`echo $result | tail -n 1 | awk -F '[:,]' '\
/errorMessages|Unauthorized/ {print "Error adding issue to JIRA"; next}\
$0 ~ "key" {\
for (i=1; i<NF;++i) {\
if (tolower($i) ~ "key") {\
   print "Issue "$(i+1)" successfuly added to JIRA"; next;\
}\
}}'`

zenity --notification --text="$feedback"
espeak-ng "$feedback" -g 5




