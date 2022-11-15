#!/bin/bash

export FS_ACTIVE_DOMAIN=1
#add domain and users
./target/debug/fscmd domain add --name teleman.me
./target/debug/fscmd user add  -u 1000 -p gxGB418oc2TVU9rt
./target/debug/fscmd user add  -u 1001 -p gxGB418oc2TVU9rt

#add gateways
./target/debug/fscmd gateway add --name kamailio_1001 --profile 2 --password 0z1*7CSUOj96nV5 --proxy 45.76.77.24 --register 45.76.77.24 --username 1001
./target/debug/fscmd gateway add --name kamailio_1002 --profile 2 --password j1U4NyRN6LtTGzp --proxy 45.76.77.24 --register 45.76.77.24 --username 1002

#add outbound route
./target/debug/fscmd route out add --condition "(1\d{10})" --gateway-id 1 --priority 100

#add inbound route
./target/debug/fscmd route in add --condition 1002 --context public --dest-extension 1000

#add ringgroup
./target/debug/fscmd rg add --group-id 6000 --name "ringgroup_1"
./target/debug/fscmd rg member add --group 1 --user 1
./target/debug/fscmd rg member add --group 1 --user 2

./target/debug/fscmd ivr add  --exten 8000 --name martin_test
./target/debug/fscmd ivr option add --digits 1 --ivr-id 1 --dest-exten 1000

./target/debug/fscmd sound-file add --name ttt.wav --path /root/play-profiles.mp3 --desc "test"
./target/debug/fscmd sound add --exten 3000 --name test --sound-file-id 1

#add conference
./target/debug/fscmd conference add --name test --exten 4000 --conference-profile-id 1

#add queue
./target/debug/fscmd queue add --exten 5002 --name test@internal
./target/debug/fscmd agent add  --name test --user-id 1
./target/debug/fscmd tier add --agent-id 1 --queue-id 1 --level 1 --position 1

export FS_ACTIVE_DOMAIN=2
./target/debug/fscmd user add -u 2000 -p gxGB418oc2TVU9rt
./target/debug/fscmd user add -u 2001 -p gxGB418oc2TVU9rt
