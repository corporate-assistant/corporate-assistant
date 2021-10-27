#!/bin/bash
declare -A myassoc
myassoc['open the terminal']='gnome-terminal'
myassoc['compose monthly status report']='github-crawler'
myassoc['file an issue to JIRA']='curl'

echo "open the terminal: ${myassoc['open the terminal']}"




#Get password from user
pass=`zenity --password`

# get custom field
result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET  https://jira.devtools.intel.com/rest/api/2/field`
#echo "Field: $result"



# Get board
result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET  https://jira.devtools.intel.com/rest/agile/1.0/board?projectKeyOrId=PADDLEQ` 


board_url=`echo $result | sed 's:.*self"\:"::' | cut -f 1 -d '"'`
 

#echo "result: $result"
echo "board_url: $board_url"

# Take the first one

# Get sprint
result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET $board_url/sprint?state=active`
echo "result: $result"
sprint_url=`echo $result | sed 's:.*self"\:"::' | cut -f 1 -d '"'`
sprint_name=`echo $result | sed 's:.*name"\:"::' | cut -f 1 -d '"'`
sprint_id=`echo $result | sed 's:.*id"\:::' | cut -f 1 -d ','`

echo "sprint Name: $sprint_name"
echo "sprint ID: $sprint_id"

# Send prepared issue entry to JIRA
#issue_name="PADDLEQ-1170"
#result=`curl -u jczaja:$pass -H "Content-Type: application/json" -X POST -d "{ \"issues\": [ \"$issue_name\" ] }" $sprint_url/issue`
#echo "result: $result"

# Prepare feedback to user
# a) Error on pattern: "errorMessages" 
# b) Success on pattern: "key"
#feedback=`echo $result | tail -n 1 | awk -F '[:,]' '\
#/errorMessages|Unauthorized/ {print "Error adding issue to JIRA"; next}\
#$0 ~ "key" {\
#for (i=1; i<NF;++i) {\
#if (tolower($i) ~ "key") {\
#   print "Issue "$(i+1)" successfuly added to JIRA"; next;\
#}\
#}}'`

#zenity --notification --text="$feedback"
#espeak-ng "$feedback" -g 5




