# problem-1081-rust-solutions

## problem description [here](https://www.beecrowd.com.br/judge/pt/problems/view/1081?origem=1)

### input sample
2
12 9
0 1
1 5
5 6
0 4
4 2
2 3
7 8
1 7
10 11
11 8
0 1
1 2
3 4
4 3
5 6
6 8
7 9
9 10

### output sample
Caso 1:
  0-1 pathR(G,1)
    1-5 pathR(G,5)
      5-6 pathR(G,6)
    1-7 pathR(G,7)
      7-8 pathR(G,8)
  0-4 pathR(G,4)
    4-2 pathR(G,2)
      2-3 pathR(G,3)

  10-11 pathR(G,11)

Caso 2:
  0-1 pathR(G,1)
    1-2 pathR(G,2)

  3-4 pathR(G,4)
    4-3

  5-6 pathR(G,6)
    6-8 pathR(G,8)

  7-9 pathR(G,9)
    9-10 pathR(G,10)

