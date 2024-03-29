#!/bin/sh -xe
cd $(dirname $0)


# arguments
NAME=$1 # the name of the service
EXE=$2 # the executable file for the service, 
START_INTERVAL=$3 # number of seconds between runs

# constants
AGENTS=~/Library/LaunchAgents
USER=$(whoami)
LABEL=$USER.$NAME
PLIST=$LABEL.plist
DIR=$(cd .. && pwd)

# make sure EXE has the correct permissions
chmod +x "$DIR/$EXE"

# compile "run"
gcc -o ../run run.c

# reset logs
rm ../logs.log || True

# test the service
../run "$EXE"

# copy info.plist
sed "s+@DIR+$DIR+g; s+@LABEL+$LABEL+g; s+@EXE+$EXE+g; s+@START_INTERVAL+$START_INTERVAL+g" info.plist >$AGENTS/$PLIST

# unload and load service
launchctl unload "$AGENTS/$PLIST"
launchctl load "$AGENTS/$PLIST"
