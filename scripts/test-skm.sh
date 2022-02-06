#!/bin/bash


#curl -X GET https://skm.trojmiasto.pl
curl -X GET 'https://skm.trojmiasto.pl/rozklad/?from=7534&to=257530&date=2022-01-26&hour=10%3A00' # %3A is a ":"
#curl -X GET https://skm.trojmiasto.pl/rozklad/ -d '
#<input type="radio" name="from" value="7534" checked="checked">
#<input type="radio" name="to" value="257530" checked="checked">
#<input type="text" name="date" value="2022-01-26">
#<input type="text" name="hour" value="10:30" class="hour" id="hour1" readonly="readonly">
#<input type="submit" value="Submit"/>
#'


#view-source:https://skm.trojmiasto.pl/rozklad/?from=7534&to=257530&date=2022-01-26&hour=10%3A00






