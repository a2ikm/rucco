#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  ./target/debug/rucco "$input" > tmp.s
  cc -o tmp.out tmp.s
  ./tmp.out
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 3 "1+2"
assert 233 "0+11+222"
assert 100 "123-23"

echo OK
