set term png small size 800,600
set output "mem-graph.png"

set ylabel "VSize"
set y2label "Size"

set ytics nomirror
set y2tics nomirror in

set yrange [0:*]
set y2range [0:*]

plot "/tmp/mem.log" using 3 with lines axes x1y1 title "vsize", \
     "/tmp/mem.log" using 2 with lines axes x1y2 title "size"