#!/bin/bash

# This is just to be able to gauge the oreder things; it's not a super-rigorous benchmark.
# Times aren't really repoducible, but the order they fall in is.

# Check they're all on the path
path_to_t=$(which t)
if [ ! -x "$path_to_t" ] ; then
   printf "t not found. Please install with 'sudo gem install t'\n" 
   exit 0
fi

path_to_tw=$(which tw)
if [ ! -x "$path_to_tw" ] ; then
   printf "tw not found. Please install with 'cargo install tw-rs'\n" 
   exit 0
fi

path_to_tweet=$(which tweet)
if [ ! -x "$path_to_tweet" ] ; then
   printf "tweet not found. Please install with 'stack install clit'\n" 
   exit 0
fi

# Get the user's 20 most recent tweets; repeat this fifty times
time(
    printf "Ruby's t\n"
    # ping once so it's fair
    t timeline hung_pope > /dev/null
    for run in {1..50}
    do
        t timeline hung_pope > /dev/null
    done
    )
time(
    printf "Rust's tw\n"
    # ping once so it's fair
    tw user -n20 > /dev/null 
    for run in {1..50}
    do
        tw user -n20 > /dev/null 
    done
    )
time(
    printf "Haskell's tweet\n"
    # ping once so it's fair
    tweet user hung_pope -n20 --color > /dev/null
    for run in {1..50}
    do
        # For Haskell's 'tweet'
        tweet user hung_pope -n20 --color > /dev/null
    done
    )
time(
    printf "Perl's oysttyer\n"
    # ping once so it's fair
    echo '/again hung_pope' | perl oysttyer.pl > /dev/null
    for run in {1..50}
    do
        # For Perl's 'oysttyer'
        echo '/again hung_pope' | perl oysttyer.pl > /dev/null
    done
    )
# TODO fav/ufav a tweet
# tw fav 844035859674529793
# tw ufav 844035859674529793
