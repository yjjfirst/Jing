#!/bin/bash

#add domain and users
./target/debug/fscmd domain add --name 45.76.77.24
./target/debug/fscmd user add -d 1 -u 1000 -p gxGB418oc2TVU9rt
./target/debug/fscmd user add -d 1 -u 1001 -p gxGB418oc2TVU9rt
./target/debug/fscmd user add -d 2 -u 2000 -p gxGB418oc2TVU9rt
./target/debug/fscmd user add -d 2 -u 2001 -p gxGB418oc2TVU9rt

#add gateways
./target/debug/fscmd gateway add --name kamailio_1001 --profile 2 --password 0z1*7CSUOj96nV5 --proxy 45.76.77.24 --register 45.76.77.24 --username 1001
./target/debug/fscmd gateway add --name kamailio_1002 --profile 2 --password j1U4NyRN6LtTGzp --proxy 45.76.77.24 --register 45.76.77.24 --username 1002

#add outbound route
./target/debug/fscmd route out add --condition "(1\d{10})" --gateway-id 1 --priority 100

#add inbound route
./target/debug/fscmd route in add --condition 1002 --context public --dest-extension 1000
