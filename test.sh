#!/bin/bash

CMD=./target/x86_64-unknown-linux-musl/debug/r9cc
TARGET=./target/tmp

mkdir -p "${TARGET}"

try() {
  expected="$1"
  input="$2"

  ${CMD} "$input" > "${TARGET}/tmp.s"
  gcc -o "${TARGET}/tmp" "${TARGET}/tmp.s"
  "${TARGET}/tmp"
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 100 100
try 2 '1+1'
try 21 '3*(9-2)'
try 14 '(3+3)+2*(5-1)'
try 2 '-3+5'
try 2 '4*-2+10'
try 1 '-2+3'

try 1 '0==0'
try 1 '23==23'
try 0 '0==1'
try 1 '10!=20'
try 0 '10!=10'

try 1 '3>=2'
try 1 '3>=3'
try 0 '2>=3'
try 1 '3>2'
try 0 '2>3'

try 1 '2<=3'
try 1 '2<=2'
try 0 '3<=2'
try 1 '2<3'
try 0 '3<2'

echo OK