#!/bin/bash

#source ./near_cli_test/init_factory_and_dao.sh

PUUID=$(near call $DCID add_proposal '{"proposal_input" : {"description": "This file is not valid anymore.", "tags":["test","invalidate"], "transaction": { "InvalidateFile": {"uuid": "5fb2ae525103eb24bb18d10779426660"} }}}' --amount 0.5 --gas 100000000000000 --accountId $CID4 | tail -n1 | tr -d '[:space:]')
echo "Created proposal UUID: $PUUID"

near call $DCID vote '{"proposal_id":'$PUUID',"vote_kind": 1}' --gas 10000000000000 --accountId $CID1
near call $DCID vote '{"proposal_id":'$PUUID',"vote_kind": 1}' --gas 10000000000000 --accountId $CID2
near call $DCID vote '{"proposal_id":'$PUUID',"vote_kind": 1}' --gas 10000000000000 --accountId $CID3
near call $DCID vote '{"proposal_id":'$PUUID',"vote_kind": 1}' --gas 10000000000000 --accountId $CID4
near call $DCID finish_proposal '{"proposal_id":'$PUUID'}' --gas 100000000000000 --accountId $CID4

near view $DCID proposal '{"proposal_id": '$PUUID'}'
near view $DCID doc_files