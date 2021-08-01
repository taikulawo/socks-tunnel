#!/bin/bash
# ./routes.sh ip 10.0.0.1 192.168.50.1 ens33 192.168.50.3
CMD=$1
TUN_GW=$2
ORIG_GW=$3
ORIG_GW_SCOPE=$4
ORIG_ST=$5

"$CMD" route del default table main
"$CMD" route add default via $TUN_GW table main
"$CMD" route add default via $ORIG_GW dev $ORIG_GW_SCOPE table default
"$CMD" rule add from $ORIG_ST table default