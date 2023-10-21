# mongo-memleak-test
Example that shows mongodb driver version 2.7 has a potential memory leak

- left axis is virtual size
- right axis is size
- probed by ```ps -C mongo-memleak-test -o pid=,size=,vsize=```

Memory usage with mongodb driver version 2.7
![Memory usage mongodb@2.7](graph/mem-graph-2.7.png?raw=true "Memory usage for mongodb driver version 2.7")

Memory usage with mongodb driver version 2.6
![Memory usage mongodb@2.6](graph/mem-graph-2.6.png?raw=true "Memory usage for mongodb driver version 2.6")

Sample code in this repository - its a simple find_one which doesnt find a document

steps to reporoduce:
- database should be empty
- cargo build --release
- ./target/release/mongo-memleak-test

Tested on:
- AMD Ryzen 5950
- 64GB or RAM
- Gentoo Linux
- stable rustc 1.73.0 (cc66ad468 2023-10-03)
- default optimizations
  
