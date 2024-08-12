#!/usr/bin/env bash


DATA_DIR_NAME="data"
SIMS=${1}
END_DATE=${2}


function download_latest_price_data {
    URL=${1}
    TOKEN=${2}

    if [[ "x${URL}" != "x" ]] && [[ "x${TOKEN}" != "x" ]]
    then
        wget -c "${URL}" -O ./${DATA_DIR_NAME}/${TOKEN}-usd.csv
    fi
}


(cat token_list.txt ; echo "") | while read TOKEN_UC URL 
do
    TOKEN=`echo ${TOKEN_UC} | sed -e 's/\(.*\)/\L\1/'`
    download_latest_price_data ${URL} ${TOKEN} 

    if [[ "x${URL}" != "x" ]] && [[ "x${TOKEN}" != "x" ]]
    then
        cargo run --release - -i ./${DATA_DIR_NAME}/${TOKEN}-usd.csv -o /dev/null -f '^(\d{4})-(\d{2})-(\d{2}).*$' -d0 -p1 -s ${SIMS} -e ${END_DATE}
    fi
done


exit 0