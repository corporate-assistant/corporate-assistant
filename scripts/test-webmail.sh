#!/bin/bash

email_template=$' {
  "Message": {
    "Subject": "Meet for lunch 2?",
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
#pass=`zenity --password`

#curl -D- -u jczaja:$pass -X POST --data @$email_file -H Content-Type:application/json https://webmail.intel.com/api/v2.0/me/sendmail

refresh_token='M.R3_BAY.CQ!VgTvCagEUtW1FU2IcvzrJRPqVtuIqFvOpNSacyBMLntnAqJVms8s6nurJazVrwEzbhx1v7YekgE26kR0lN9I3EWZaYQviIddQxF3hG2Wte4lkm6LEqbF9ru09C7AM3tQcuxEdc1cp4RloR!tD6jGbNhZl3!8cl2jhrRYZ3*PXNabUHQ*SQadfWsIi4TL5ftdXsguuadHzhtLW7Toh*Cw5O4Tt1sGjd!NJAFODSk1VAANy7!X*2pbeR4zz2leP1kAnxNmHSvmrG8Tq6cmUKBoJYR4fKa0OI3cmtmfGT9nBWGfbgrGWCzIyOta7ZTge2vOTZsIy9gOWEtHk!92u*cdw7xWvrxPBGOIbiEJwxUpOgzR*9RWDS8tJ2aY2LQQ1Ls3w5Kt6lhftzyAwik19ixLiIhgPY9XWaF6KZrmCC3*0vaUel9mIqdvBb*LAchhTg3E6EWvojNm42kA0rq9YuD4HgrZ1sSt!33Zkm9MD7A2R0GsRyelECXju7N1qEkv9BvPR!rSXSIv5pcM7OP7xr!MtxJ272y6YxCxuMvRRQ1xoVSB40*xdGHJ2rC0hGXeU2sQefdw0O2HQsHrmiJgwRr0X65WSBW9NxT5h56TGUT0cqqVzkMohFO0uoTV4OusfITL!13EXEG2m7LzuKiwfFNI$'


curl -X POST https://login.windows.net/common/oauth2/v2.0/token HTTP/1.1


#curl -X POST -H "Authorization: Bearer EwBYA+l3BAAUFFpUAo7J3Ve0bjLBWZWCclRC3EoAAd8nhpmuecEVXcEwmVd1ouecs/EM/kruoMwDo/f1yrtUBpUCVeAaKSfKfOjMT19wFDdWeG5ZoZkgOlY3NjZcLA9VEXVjwWcpILJCSAfNVYjOElFp44g5f9yq0hdR2aTVhLGWqv9cY3crbenaxFD88BYUa0DHfm9cxHyA68NM0iRD+ON8Fvsn/iNbzkaleG6S4cdpVSn13l/syRY/SuBfFbN+6Uf01P1pGJOCxQk6cJgXsPIUY5ZYbAVvowtgUKaSzERze0laIioNCLUTuvr7KtOnOst/hTPoEgbE0KFpbFDRt11mbC0l7HTqiUM3Vn4NYkVZdraRZJhnvy8oy7jfLqADZgAACFSiqzEPhQKNKALWrI8zQITExnDmOFTMGrxdaoU+PiRYQBzFkV/H5Ftbntw/yum6I8YyWkCVVWsFDXCY8dh4tAk6WoW62WijzDZdpNq6ccUW/JHPBx2lwOsSvvBeaiEUXpD4LFG/lU/70kXZ0hnRTsD28zLoVxHOOEu65eJH6/XBsTKFtPqMZaJRxa7NUuyJEd0+amekRkr6JVH8b5wDZcHh5Nu7N6l0ST2mZuy54U3QH4o021Lf51W4tGDNDomQ1VZOikjAyECdIX9/06yBItvUITmKKN1l8V1n4ITctm6wI3hmd9hQ/xbimEIo4V3o811VtnpSgHtcI25yDpsg1GScSCa8iHszaBTip8fwqG2Na7zlbZrJlJlUgehFOSuVhOC8ngVxzGUAQ6bQDafSX0AN0PTR7OX4v664l6A+XgchGJHXVQJdlx6k2akPs3w7t7Xd9+QM1tf9bSWe8MfZj60ygBIB4H6ZLlFi6kpkflxjlbgM2w15LTj5sjo+iuU6mNGXXcRe1AnL9yYELLVd6umkc3XrY5ERLrX/l10Mqbpg6drCmtmPmDbaCVI5W+0R8YOIzFqodLLrHe4B1OIWsNlxRTBFXNzLAsyu/XXANwVir8weOWC6NIzzXGIhQzxMAFHzIyLHu+LX6MVIK/28Z5un+D2T6WexPawR3AH9dXRpLxQu9ZaxyvhvbJcZCNqMy1o3o+qgYnu23JqciNRgPXjCtlk+jCfA8fLysyzsiYJ+8WxZAg==" -H "Accept: application/json; odata.metadata=none" -H "Content-Type: application/json" -d @$email_file https://outlook.office.com/api/v2.0/me/sendmail




