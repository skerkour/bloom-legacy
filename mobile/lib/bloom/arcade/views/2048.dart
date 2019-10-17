import 'dart:async';
import 'package:flutter/material.dart';
import '../widgets/game.dart';
import '../widgets/grid.dart';
import '../widgets/mycolor.dart';
import '../widgets/tile.dart';

class A2048View extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    // TODO(x): implement createState
    return _A2048ViewState();
  }
}

class _A2048ViewState extends State<A2048View> {
  List<List<int>> grid = <List<int>>[];
  List<List<int>> gridNew = <List<int>>[];
  int score = 0;
  bool isgameOver = false;
  bool isgameWon = false;

  List<Widget> getGrid(double width, double height) {
    final List<Widget> grids = <Widget>[];
    for (int i = 0; i < 4; i++) {
      for (int j = 0; j < 4; j++) {
        final int num = grid[i][j];
        String number;
        int color;
        if (num == 0) {
          color = MyColor.emptyGridBackground;
          number = '';
        } else if (num == 2 || num == 4) {
          color = MyColor.gridColorTwoFour;
          number = '$num';
        } else if (num == 8 || num == 64 || num == 256) {
          color = MyColor.gridColorEightSixtyFourTwoFiftySix;
          number = '$num';
        } else if (num == 16 || num == 32 || num == 1024) {
          color = MyColor.gridColorSixteenThirtyTwoOneZeroTwoFour;
          number = '$num';
        } else if (num == 128 || num == 512) {
          color = MyColor.gridColorOneTwentyEightFiveOneTwo;
          number = '$num';
        } else {
          color = MyColor.gridColorWin;
          number = '$num';
        }
        double size;
        final String n = '$number';
        switch (n.length) {
          case 1:
          case 2:
            size = 40.0;
            break;
          case 3:
            size = 30.0;
            break;
          case 4:
            size = 20.0;
            break;
        }
        grids.add(Tile(number, width, height, color, size));
      }
    }
    return grids;
  }

  void handleGesture(int direction) {
    /*

      0 = up
      1 = down
      2 = left
      3 = right

    */
    bool flipped = false;
    bool played = true;
    bool rotated = false;
    if (direction == 0) {
      setState(() {
        grid = transposeGrid(grid);
        grid = flipGrid(grid);
        rotated = true;
        flipped = true;
      });
    } else if (direction == 1) {
      setState(() {
        grid = transposeGrid(grid);
        rotated = true;
      });
    } else if (direction == 2) {
    } else if (direction == 3) {
      setState(() {
        grid = flipGrid(grid);
        flipped = true;
      });
    } else {
      played = false;
    }

    if (played) {
      print('playing');
      final List<List<int>> past = copyGrid(grid);
      print('past $past');
      for (int i = 0; i < 4; i++) {
        setState(() {
          final List<dynamic> result = operate(grid[i], score);
          score = result[0];
          print('score in set state $score');
          grid[i] = result[1];
        });
      }
      setState(() {
        grid = addNumber(grid, gridNew);
      });
      final bool changed = compare(past, grid);
      print('changed $changed');
      if (flipped) {
        setState(() {
          grid = flipGrid(grid);
        });
      }

      if (rotated) {
        setState(() {
          grid = transposeGrid(grid);
        });
      }

      if (changed) {
        setState(() {
          grid = addNumber(grid, gridNew);
          print('is changed');
        });
      } else {
        print('not changed');
      }

      final bool gameover = isGameOver(grid);
      if (gameover) {
        print('game over');
        setState(() {
          isgameOver = true;
        });
      }

      final bool gamewon = isGameWon(grid);
      if (gamewon) {
        print('GAME WON');
        setState(() {
          isgameWon = true;
        });
      }
      print(grid);
      print(score);
    }
  }

  @override
  void initState() {
    // TODO(x): implement initState
    grid = blankGrid();
    gridNew = blankGrid();
    addNumber(grid, gridNew);
    addNumber(grid, gridNew);
    super.initState();
  }

  Future<String> getHighScore() async {
    score = 0;
    return score.toString();
  }

  @override
  Widget build(BuildContext context) {
    // TODO(x): implement build
    final double width = MediaQuery.of(context).size.width;
    final double gridWidth = (width - 80) / 4;
    final double gridHeight = gridWidth;
    final double height = 30 + (gridHeight * 4) + 10;

    return Scaffold(
      appBar: AppBar(
        centerTitle: true,
        title: const Text(
          '2048',
          style: TextStyle(fontSize: 25.0, fontWeight: FontWeight.bold),
        ),
        backgroundColor: Color(MyColor.gridBackground),
      ),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            children: <Widget>[
              Padding(
                padding: const EdgeInsets.all(10.0),
                child: Container(
                  width: 200.0,
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(20.0),
                    color: Color(MyColor.gridBackground),
                  ),
                  height: 82.0,
                  child: Column(
                    children: <Widget>[
                      const Padding(
                        padding: EdgeInsets.only(top: 10.0, bottom: 2.0),
                        child: Text(
                          'Score',
                          style: TextStyle(
                              fontSize: 15.0,
                              color: Colors.white70,
                              fontWeight: FontWeight.bold),
                        ),
                      ),
                      Padding(
                        padding: const EdgeInsets.only(bottom: 10.0),
                        child: Text(
                          '$score',
                          style: const TextStyle(
                              fontSize: 20.0,
                              color: Colors.white,
                              fontWeight: FontWeight.bold),
                        ),
                      )
                    ],
                  ),
                ),
              ),
              Container(
                height: height,
                child: Stack(
                  children: <Widget>[
                    Padding(
                      padding: const EdgeInsets.all(10.0),
                      child: GestureDetector(
                        child: GridView.count(
                          primary: false,
                          crossAxisSpacing: 10.0,
                          mainAxisSpacing: 10.0,
                          crossAxisCount: 4,
                          children: getGrid(gridWidth, gridHeight),
                        ),
                        onVerticalDragEnd: (DragEndDetails details) {
                          //primaryVelocity -ve up +ve down
                          if (details.primaryVelocity < 0) {
                            handleGesture(0);
                          } else if (details.primaryVelocity > 0) {
                            handleGesture(1);
                          }
                        },
                        onHorizontalDragEnd: (DragEndDetails details) {
                          //-ve right, +ve left
                          if (details.primaryVelocity > 0) {
                            handleGesture(2);
                          } else if (details.primaryVelocity < 0) {
                            handleGesture(3);
                          }
                        },
                      ),
                    ),
                    isgameOver
                        ? Container(
                            height: height,
                            color: Color(MyColor.transparentWhite),
                            child: Center(
                              child: Text(
                                'Game over!',
                                style: TextStyle(
                                    fontSize: 30.0,
                                    fontWeight: FontWeight.bold,
                                    color: Color(MyColor.gridBackground)),
                              ),
                            ),
                          )
                        : const SizedBox(),
                    isgameWon
                        ? Container(
                            height: height,
                            color: Color(MyColor.transparentWhite),
                            child: Center(
                              child: Text(
                                'You Won!',
                                style: TextStyle(
                                    fontSize: 30.0,
                                    fontWeight: FontWeight.bold,
                                    color: Color(MyColor.gridBackground)),
                              ),
                            ),
                          )
                        : const SizedBox(),
                  ],
                ),
                color: Color(MyColor.gridBackground),
              ),
              Padding(
                padding: const EdgeInsets.all(10.0),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: <Widget>[
                    Container(
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(20.0),
                        color: Color(MyColor.gridBackground),
                      ),
                      child: Padding(
                        padding: const EdgeInsets.all(10.0),
                        child: IconButton(
                          iconSize: 35.0,
                          icon: Icon(
                            Icons.refresh,
                            color: Colors.white70,
                          ),
                          onPressed: () {
                            setState(() {
                              grid = blankGrid();
                              gridNew = blankGrid();
                              grid = addNumber(grid, gridNew);
                              grid = addNumber(grid, gridNew);
                              score = 0;
                              isgameOver = false;
                              isgameWon = false;
                            });
                          },
                        ),
                      ),
                    ),
                    //   Container(
                    //     decoration: BoxDecoration(
                    //       borderRadius: BorderRadius.circular(20.0),
                    //       color: Color(MyColor.gridBackground),
                    //     ),
                    //     child: Padding(
                    //       padding: EdgeInsets.all(10.0),
                    //       child: Column(
                    //         children: <Widget>[
                    //           Text(
                    //             'High Score',
                    //             style: TextStyle(
                    //                 color: Colors.white70,
                    //                 fontWeight: FontWeight.bold),
                    //           ),
                    //           FutureBuilder<String>(
                    //             future: getHighScore(),
                    //             builder: (ctx, snapshot) {
                    //               if (snapshot.hasData) {
                    //                 return Text(
                    //                   snapshot.data,
                    //                   style: TextStyle(
                    //                       color: Colors.white,
                    //                       fontWeight: FontWeight.bold),
                    //                 );
                    //               } else {
                    //                 return Text(
                    //                   '0',
                    //                   style: TextStyle(
                    //                       color: Colors.white,
                    //                       fontWeight: FontWeight.bold),
                    //                 );
                    //               }
                    //             },
                    //           )
                    //         ],
                    //       ),
                    //     ),
                    //   )
                  ],
                ),
              )
            ],
          ),
        ),
      ),
    );
  }
}
