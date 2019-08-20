if [[ $PWD/ != */scripts/ ]]; then
    cd scripts
fi
cat assets/disposable_emails.json | jq '.[]' -r > /tmp/disposable_emails.txt
cat assets/disposable_email_blacklist.conf >> /tmp/disposable_emails.txt
sort -u /tmp/disposable_emails.txt > ../assets/disposable_email_domains.txt
rm -rf /tmp/disposable_emails.txt
