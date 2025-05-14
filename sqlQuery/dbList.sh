#!/bin/bash




database=$(cat ~/scripts/sqlLog/dbList.md | gum filter)

function db() {
    $ns -u low ${database}
}

db
