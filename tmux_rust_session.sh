#!/bin/sh
#
# Setup a work space called `work` with two windows
# first window has 3 panes. 
# The first pane set at 65%, split horizontally, set to api root and running vim
# pane 2 is split at 25% and running redis-server 
# pane 3 is set to api root and bash prompt.
# note: `api` aliased to `cd ~/path/to/work`
#
session=${PWD##*/}

# set up tmux
tmux start-server

# create a new tmux session, starting vim from a saved session in the new window
tmux new-session -d -s $session -n nvim

# Select pane 1, set dir to api, run vim
tmux selectp -t 0
tmux send-keys "nvim -c 'SessionManager load_current_dir_session'" C-m 

# Split pane 1 horizontal by 65%
tmux splitw -h -p 10
tmux resize-pane -R 30
tmux send-keys "cargo watch -c -w src -x test" C-m 

# Select pane 0
tmux selectp -t 0

# return to main vim window
tmux select-window -t $session:0

# Finished setup, attach to the tmux session!
tmux attach-session -t $session
