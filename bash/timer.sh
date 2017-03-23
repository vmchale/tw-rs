#!/bin/bash
# set variables for each command!!
# FIXME figure out how to 

# Get the user's 20 most recent tweets; repeat ten times
time(
    printf "Ruby's t\n"
    # ping once so it's fair
    t timeline hung_pope > /dev/null
    for run in {1..10}
    do
        t timeline hung_pope > /dev/null
    done
    )
time(
    printf "Rust's tw\n"
    # ping once so it's fair
    tw user -n20 > /dev/null 
    for run in {1..10}
    do
        tw user -n20 > /dev/null 
    done
    )
time(
    printf "Haskell's tweet\n"
    # ping once so it's fair
    tweet user hung_pope -n20 --color > /dev/null
    for run in {1..10}
    do
        # For Haskell's 'tweet'
        tweet user hung_pope -n20 --color > /dev/null
    done
    )
