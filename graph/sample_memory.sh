rm /tmp/mem.log
while true; do
ps -C mongo-memleak-test -o pid=,size=,vsize= >> /tmp/mem.log
gnuplot show_mem.plt
sleep 1
done