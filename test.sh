exec cargo run <<EOF
cls

10 sum=0
20 i=1
30 sum=sum+i
40 i=i+1
50 if i<10 then goto 30
60 print sum

list

run
EOF
