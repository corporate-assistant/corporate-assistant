#!/bin/bash

username=jczaja
candidate_model=/home/jczaja/DEEPSPEECH/jacek-04-02-2021.pbmm
scorer=/home/jczaja/DEEPSPEECH/deepspeech-0.9.3-models.scorer

# callbacks
function create_msr {
    local date_to=`date +%d.%m.%Y` 
    local date_from=`date +%d.%m.%Y --date='1 month ago'`
    local github_crawler="/home/jczaja/DEEPSPEECH/github-crawler/target/release/github-crawler --user=$username --from=$date_from --to=$date_to --repo=Paddle --behind-proxy"
    espeak-ng "Composing monthly status report" -g 5  
    echo "Getting PRs info from Github"
    $github_crawler | tail -n +6 | gvim -
}

function create_wsr {
    local date_to=`date +%d.%m.%Y` 
    local date_from=`date +%d.%m.%Y --date='1 week ago'`
    local github_crawler="/home/jczaja/DEEPSPEECH/github-crawler/target/release/github-crawler --user=$username --from=$date_from --to=$date_to --repo=Paddle --behind-proxy"
    espeak-ng "Composing weekly status report" -g 5  
    echo "Getting PRs info from Github"
    $github_crawler | tail -n +6 | gvim -
}

function open_terminal {
    espeak-ng "Opening terminal" -g 5  
    echo "Opening terminal"
    gnome-terminal 
}

function open_lunch_menu {
    espeak-ng "Opening lunch menu" -g 5  
    echo "Opening lunch menu"
#    xdg-open https://enjoyyourmeal.pl/menu/restaurant/id/90
    xdg-open https://enjoyyourmeal.pl/menu/restaurant/id/36
#    xdg-open https://enjoyyourmeal.pl/menu/restaurant/id/93
}

function open_shuttle_bus_schedule {
    espeak-ng "Opening the shuttle bus schedule" -g 5  
    echo "Opening the shuttle bus schedule"
    xdg-open https://sp2010.amr.ith.intel.com/sites/IGK/gptw/SiteAssets/SitePages/IGKShuttle/rozklad.JPG
}

function open_holidays {
    espeak-ng "Opening the holidays request form" -g 5  
    echo "Opening the holdiday request form"
    xdg-open 'https://www.myworkday.com/intel/d/task/2997$275.htmld'
}

function create_jira_issue {
  local issue_template=$'{"fields":{
    "project":{
      "key": "PADDLEQ"},
      "summary": "Test issue (disregard)",
      "description": "Test",
      "issuetype": {
        "name": "Task"}}}'
  local issue_file=/tmp/`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 8`

  espeak-ng "Please type your password and edit jira issue template" -g 5  
  # 1. Get active sprint if available

  ## 1.1 Get password from user
  local pass=`zenity --password`

  ## 1.2 Get active sprint (Name and URL)
  ### 1.2.1 Get board (scrum)
  result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET  https://jira.devtools.intel.com/rest/agile/1.0/board?projectKeyOrId=PADDLEQ` 
  board_url=`echo $result | sed 's:.*self"\:"::' | cut -f 1 -d '"'`
  ### 1.2.2 Get active sprint
  result=`curl -u jczaja:$pass -H Content-Type: application/json -X GET $board_url/sprint?state=active`
  sprint_url=`echo $result | sed 's:.*self"\:"::' | cut -f 1 -d '"'`
  sprint_name=`echo $result | sed 's:.*name"\:"::' | cut -f 1 -d '"'`

  # Put template into temporary file for user to edit
  echo "$issue_template" > $issue_file 
  # Add Sprint name to template file
  echo "SPRINT:\"$sprint_name\" # Delete this line if issue is not to be assigned to sprint" >> $issue_file 
  gvim -f $issue_file 
  # Take out the line with SPRINT if exists
  local sprint_name=`cat $issue_file |  awk -F '["]' '\
  /SPRINT:/ {\
     print $2; next;\
  }'`

  # Send prepared issue entry to JIRA
  local result=`curl -D- -u $username:$pass -X POST --data @$issue_file -H Content-Type:application/json https://jira.devtools.intel.com/rest/api/2/issue/`
  
  # TODO make a project KEY (PADDLEQ) a variable
  # TODO: connectivty issues should be also reported here
  # Prepare feedback to user
  # a) Error on pattern: "errorMessages" or "Unauthorized" 
  # b) Success on pattern: "key"
  local feedback=`echo $result | tail -n 1 | sed 's:.*key"\:":key"\::'| awk -F '[:,]' '\
  /errorMessages|Unauthorized/ {print "Error adding issue to JIRA"; next}\
  $0 ~ "key" {\
  for (i=1; i<NF;++i) {\
  if (tolower($i) ~ "key") {\
     print "Issue "$(i+1)" successfully added to JIRA"; next;\
  }\
  }}'`


  # Adding JIRA issue to sprint if only issue was successfuly created and sprint_name is not empty
  if echo "$feedback" | grep -q "successfully" &&  [ ! -z "$sprint_name" ]; then
    ## Send Issue to active sprint
    issue_name=`echo $feedback | cut -f 2 -d ' '`
    echo "issuename:$issue_name"
    result=`curl -u jczaja:$pass -H "Content-Type: application/json" -X POST -d "{ \"issues\": [ \"$issue_name ] }" $sprint_url/issue`
    # If empty result then ok
    # TODO: check if variable empty then ok  and print / say message on adding to sprint
    #       if variale is having content then report an error
    feedback="Issue $issue_name successfuly added to Sprint $sprint_name in JIRA"
    zenity --notification --text="$feedback"
    espeak-ng "$feedback" -g 5
  else 
    zenity --notification --text="$feedback"
    espeak-ng "$feedback" -g 5
  fi

  # cleanup
  rm $issue_file

}

# Intentions
declare -A intentions
intentions['compose monthly status report']='create_msr'
intentions['compose my monthly status report']='create_msr'
intentions['create monthly status report']='create_msr'
intentions['create my monthly status report']='create_msr'
intentions['compose weekly status report']='create_wsr'
intentions['compose my weekly status report']='create_wsr'
intentions['create weekly status report']='create_wsr'
intentions['create my weekly status report']='create_wsr'
intentions['open terminal']='open_terminal'
intentions['open the terminal']='open_terminal'
intentions['create an issue']='create_jira_issue'
intentions['compose an issue']='create_jira_issue'
intentions['file an issue']='create_jira_issue'
intentions['file issue']='create_jira_issue'
intentions['open the shuttle bus schedule']='open_shuttle_bus_schedule '
intentions['i want holidays']='open_holidays'
intentions['i want vacations']='open_holidays'
intentions['i want to book holidays']='open_holidays'
intentions['i want to book vacations']='open_holidays'
intentions['i want to request holidays']='open_holidays'
intentions['i want to request vacations']='open_holidays'
intentions['give me holidays']='open_holidays'
intentions['show me lunch menu']='open_lunch_menu'
intentions['show me the lunch menu']='open_lunch_menu'
intentions['what is for lunch']='open_lunch_menu'
intentions['where should i eat']='open_lunch_menu'
intentions['what should i eat']='open_lunch_menu'
intentions['open the lunch menus']='open_lunch_menu'
intentions['open the lunch menu']='open_lunch_menu'
intentions['show me lunch menus']='open_lunch_menu'
intentions['show me the lunch menu']='open_lunch_menu'

streaming=0 # Possible values: 1,0

if ((streaming == 1)); then   # streaming
# Get Output (one or many transcriptions and pick last one)
transcription=`python3 $(dirname $0)/mic_vad_streaming.py --model $candidate_model --scorer=$scorer` 
transcription=`echo $transcription | grep "Recognized" | sed 's:.*Recognized\:::' `
echo "Transcription: $transcription"
else # No streaming 
# Report
#espeak-ng "I'm listening.." -g 5  
# record 4 seconds or less if SIGKILL
#filename=/tmp/`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 8`
#arecord -r 16000 -d 4 -f S16_LE $filename.wav
transcription=`LD_LIBRARY_PATH=/home/jczaja/DEEPSPEECH/deepspeech-native/ /home/jczaja/DEEPSPEECH/corporate-assistant/target/release/transcriber --model=$candidate_model --scorer=$scorer 2>/dev/null`

# generation of transcription
#transcription=`deepspeech --model $candidate_model --scorer=$scorer --audio $filename.wav 2>/dev/null`
echo "Transcription: $transcription"
#rm $filename.wav
fi



if [ ${intentions[$transcription]} ]; then
  echo "calling: "${intentions[$transcription]}
  ${intentions[$transcription]}
else
    echo "Strings are not equal."
    zenity --notification --text="transcription: $transcription"
    espeak-ng "I do not understand" -g 5
fi






