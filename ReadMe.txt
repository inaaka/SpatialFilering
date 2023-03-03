空間フィルタリング用のプログラムです。自己責任で使ってください。

コードは念のため添付しておきますがRUSTをあまり理解してないので冗長だったり読みずらかったりすると思います。ごめんなさい。

[使い方]
1. cnncui.exeを起動
2. [INPUT]の文の後に入力画像，入力フィルタ(CSV)，出力ファイルを入力してください．

例）
> cnncui.exe
[INPUT ]input image : ./img/image.jpg
[STATUS]open image : Done
[INPUT ]input filter : ./filter/smoothing1.csv
[STATUS]open csv : Done
[STATUS]read csv : Done
[INFO  ]filter :
0.04, 0.04, 0.04, 0.04, 0.04,
0.04, 0.04, 0.04, 0.04, 0.04,
0.04, 0.04, 0.04, 0.04, 0.04,
0.04, 0.04, 0.04, 0.04, 0.04,
0.04, 0.04, 0.04, 0.04, 0.04,
[STATUS]spatial filtering : Done
[INPUT ]output image : ./dst/flat1.png
[STATUS]write image : Done