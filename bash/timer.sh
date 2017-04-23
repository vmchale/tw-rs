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
   printf "tweet not found. Please install with 'stack install tweet-hs'\n" 
   exit 0
fi

path_to_bench=$(which bench)
if [ ! -x "$path_to_bench" ] ; then
   printf "tweet not found. Please install with 'stack install bench'\n" 
   exit 0
fi

# ping once so it's fair
printf "ping once to initialize..."
t timeline realDonaldTrump > /dev/null

printf "Ruby's t\n"
bench "t timeline realDonaldTrump" --output t.html

printf "Haskell's tweet\n"
bench "tweet user realDonaldTrump" --output tweet.html

printf "Rust's tw\n"
bench "tw user realDonaldTrump" --output tw.html

printf "Perl's oysttyer\n"
bench "echo '/again realDonaldTrump' | perl oysttyer.pl" --output oysttyer.html
