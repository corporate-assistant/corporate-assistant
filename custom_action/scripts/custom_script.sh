#!/bin/bash
gvim -S /home/jczaja/Paddle/cache.vim
#gnome-terminal -e 'cd /home/jczaja/Paddle'
gnome-terminal -e 'sh -c "ssh bduser@broncos-clx01.jf.intel.com"' #"sudo su jczaja"
gnome-terminal -- tmux
