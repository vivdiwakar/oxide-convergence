#!/usr/bin/env bash


SIMS=${1}
END_DATE=${2}


function download_latest_price_data {
    URL=${1}
    TOKEN=${2}
    wget -c "${URL}" -O ./tmp/${TOKEN}-usd.csv
}


(cat token_list.txt ; echo "") | while read TOKEN_UC URL 
do
    TOKEN=`echo ${TOKEN_UC} | sed -e 's/\(.*\)/\L\1/'`
    download_latest_price_data ${URL} ${TOKEN} 
    cargo run --release - -i ./tmp/${TOKEN}-usd.csv -o /dev/null -f '^(\d{4})-(\d{2})-(\d{2}).*$' -d0 -p1 -s ${SIMS} -e ${END_DATE}
done


exit 0