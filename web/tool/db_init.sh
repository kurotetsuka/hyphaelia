#!/bin/bash

db_user=hyph
db_pass=hyph

mysql -u $db_user -p$db_pass < src/create_tables.sql
mysql -u $db_user -p$db_pass < src/init_tables.sql
