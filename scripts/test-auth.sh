#!/bin/bash

email_template=$' {
  "Message": {
    "Subject": "Meet for lunch?",
    "Body": {
      "ContentType": "Text",
      "Content": "The new cafeteria is open."
    },
    "ToRecipients": [
      {
        "EmailAddress": {
          "Address": "jacek.czaja@intel.com"
        }
      }
    ]
  },
  "SaveToSentItems": "false"
}'



email_file=/tmp/`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 8`

echo "$email_template" > $email_file 

## 1.1 Get password from user
pass=`zenity --password`

intel_auth=46c98d88-e344-4ed4-8496-4ed7712e255d

curl -D- -u jczaja:$pass -X POST --data @$email_file -H Content-Type:application/json https://webmail.intel.com/api/v2.0/me/sendmail




