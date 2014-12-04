#!/bin/bash

db_user=hyph
db_pass=hyph

echo "drop database hyph;" | mysql -u $db_user -p$db_pass
