User call "create custom action" then custom action editor starts . It is needed to get some script for action as well as phrase recorded . Then script with phrase are stored in a separate toml file.

example toml:

[[custom_actions]]
 phrase = "open the terminal"
 script = '''gnome-terminal -- tmux'''

So Once we got custom actions loaded We iterate through it and register each time Executor object
Each script is sotred as string so for linux we use eval to execute it script

When user want to create action  then there will be field "custom phrase" and "load" that ould load script with that key phrase as well as "save" to overwrite existing script with new one


When user is to record phrase then upon clicking on button label of butten will change from Record to "Recording...." . After 3 seconds recording is concluded. label is  reverted and STT would process recorded samples
and result will be put into text box of phrase

Internally message is send upon clicking on Record button and change of label is executed and sending new message to start recording. recording cannot start inside on click handler as message changing label has to be processed.
